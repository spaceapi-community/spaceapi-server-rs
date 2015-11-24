//! Handlers for the server.

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};
use iron::prelude::*;
use iron::{status, headers, middleware};
use iron::modifiers::Header;
use router::Router;
use redis::ConnectionInfo;

use urlencoded;

use ::api;
use ::api::optional::Optional;
use ::sensors;
use ::modifiers;


#[derive(Debug)]
struct ErrorResponse {
    reason: String,
}

impl ToJson for ErrorResponse {
    /// Serialize an ErrorResponse object into a proper JSON structure.
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("status".to_string(), "error".to_json());
        d.insert("reason".to_string(), self.reason.to_json());
        Json::Object(d)
    }
}

pub struct ReadHandler {
    status: api::Status,
    redis_connection_info: ConnectionInfo,
    sensor_specs: sensors::SafeSensorSpecs,
    status_modifiers: Vec<Box<modifiers::StatusModifier>>,
}

impl ReadHandler {
    pub fn new(status: api::Status,
               redis_connection_info: ConnectionInfo,
               sensor_specs: sensors::SafeSensorSpecs,
               status_modifiers: Vec<Box<modifiers::StatusModifier>>)
               -> ReadHandler {
        ReadHandler {
            status: status,
            redis_connection_info: redis_connection_info,
            sensor_specs: sensor_specs,
            status_modifiers: status_modifiers,
        }
    }

    fn build_response_json(&self) -> Json {

        // Create a mutable copy of the status struct
        let mut status_copy = self.status.clone();

        // Process registered sensors
        for sensor_spec in self.sensor_specs.lock().unwrap().iter() {

            match sensor_spec.get_sensor_value(&self.redis_connection_info) {

                // Value could be read successfullly
                Ok(value) => {
                    if status_copy.sensors.is_absent() {
                        status_copy.sensors = Optional::Value(api::Sensors {
                            people_now_present: Optional::Absent,
                            temperature: Optional::Absent,
                        });
                    }
                    sensor_spec.template.to_sensor(&value, &mut status_copy.sensors.as_mut().unwrap());
                },

                // Value could not be read, do error logging
                Err(err) => {
                    match err {
                        sensors::SensorError::Redis(e) => {
                            warn!("Could not retrieve key '{}' from Redis, omiting the sensor", &sensor_spec.data_key);
                            debug!("Error: {:?}", e);
                        },
                        _ =>  error!("Could not retrieve sensor '{}', unknown error.", &sensor_spec.data_key)
                    }
                },
            }
        }

        for status_modifier in self.status_modifiers.iter() {
            status_modifier.modify(&mut status_copy);
        }

        // Serialize to JSON
        status_copy.to_json()
    }
}

impl middleware::Handler for ReadHandler {

    /// Return the current status JSON.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        info!("{} /{} from {}", req.method, req.url.path[0], req.remote_addr);

        // Get response body
        let body = self.build_response_json().to_string();

        // Create response
        let response = Response::with((status::Ok, body))
            // Set headers
            .set(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())))
            .set(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])))
            .set(Header(headers::AccessControlAllowOrigin::Any));

        Ok(response)
    }

}


pub struct UpdateHandler {
    redis_connection_info: ConnectionInfo,
    sensor_specs: sensors::SafeSensorSpecs,
}


impl UpdateHandler {
    pub fn new(redis_connection_info: ConnectionInfo, sensor_specs: sensors::SafeSensorSpecs)
               -> UpdateHandler {
        UpdateHandler {
            redis_connection_info: redis_connection_info,
            sensor_specs: sensor_specs,
        }
    }

    /// Update sensor value in Redis
    fn update_sensor(&self, sensor: &str, value: &str) -> Result<(), sensors::SensorError> {
        // Validate sensor
        let sensor_specs = self.sensor_specs.lock().unwrap();
        let sensor_spec = try!(sensor_specs.iter()
                               .find(|&spec| spec.data_key == sensor)
                               .ok_or(sensors::SensorError::UnknownSensor(sensor.to_string())));

        // Store data
        sensor_spec.set_sensor_value(&self.redis_connection_info, value)
    }

    /// Build an OK response with the `HTTP 204 No Content` status code.
    fn ok_response(&self) -> Response {
        Response::with((status::NoContent))
            // Set headers
            .set(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())))
            .set(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])))
            .set(Header(headers::AccessControlAllowOrigin::Any))
    }

    /// Build an error response with the specified `error_code` and the specified `reason` text.
    fn err_response(&self, error_code: status::Status, reason: &str) -> Response {
        let error = ErrorResponse { reason: reason.to_string() };
        Response::with((error_code, error.to_json().to_string()))
            // Set headers
            .set(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())))
            .set(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])))
            .set(Header(headers::AccessControlAllowOrigin::Any))
    }

}

impl middleware::Handler for UpdateHandler {

    /// Update the sensor, return correct status code.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        // TODO: create macro for these info! invocations.
        info!("{} /{} from {}", req.method, req.url.path[0], req.remote_addr);

        // Get sensor name
        let sensor_name;
        {
            // TODO: Properly propagate errors
            let params = req.extensions.get::<Router>().unwrap();
            sensor_name = params.find("sensor").unwrap().to_string();
        }

        // Get sensor value
        let sensor_value;
        {
            let params = req.get_ref::<urlencoded::UrlEncodedBody>().unwrap();
            sensor_value = match params.get("value") {
                Some(ref values) =>  match values.len() {
                    1 => values[0].to_string(),
                    _ => return Ok(self.err_response(status::BadRequest, "Too many values specified")),
                },
                None => return Ok(self.err_response(status::BadRequest, "\"value\" parameter not specified")),
            }
        }

        // Update values in Redis
        if let Err(e) = self.update_sensor(&sensor_name, &sensor_value) {
            error!("Updating sensor value for sensor \"{}\" failed: {:?}", &sensor_name, e);
            let response = match e {
                sensors::SensorError::UnknownSensor(sensor) =>
                    self.err_response(status::BadRequest, &format!("Unknown sensor: {}", sensor)),
                sensors::SensorError::Redis(_) =>
                    self.err_response(status::InternalServerError, "Updating values in datastore failed"),
            };
            return Ok(response)
        };

        // Create response
        Ok(self.ok_response())
    }

}
