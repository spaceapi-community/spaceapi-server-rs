//! Handlers for the server.

// use iron::modifiers::Header;
// use iron::prelude::*;
// use iron::{headers, middleware, status};
use log::{debug, error, info, warn};
// use router::Router;
use serde::ser::{Serialize, SerializeMap, Serializer};

use crate::api;
use crate::modifiers;
use crate::sensors;
use crate::types::RedisPool;

#[derive(Debug)]
struct ErrorResponse {
    reason: String,
}

impl Serialize for ErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("status", "error")?;
        map.serialize_entry("reason", &self.reason)?;
        map.end()
    }
}

pub(crate) struct ReadHandler {
    status: api::Status,
    redis_pool: RedisPool,
    sensor_specs: sensors::SafeSensorSpecs,
    status_modifiers: Vec<Box<dyn modifiers::StatusModifier>>,
}

impl ReadHandler {
    pub(crate) fn new(
        status: api::Status,
        redis_pool: RedisPool,
        sensor_specs: sensors::SafeSensorSpecs,
        status_modifiers: Vec<Box<dyn modifiers::StatusModifier>>,
    ) -> ReadHandler {
        ReadHandler {
            status,
            redis_pool,
            sensor_specs,
            status_modifiers,
        }
    }

    fn build_response_json(&self) -> String {
        // Create a mutable copy of the status struct
        let mut status_copy = self.status.clone();

        // Process registered sensors
        for sensor_spec in self.sensor_specs.iter() {
            match sensor_spec.get_sensor_value(&self.redis_pool) {
                // Value could be read successfullly
                Ok(value) => {
                    if status_copy.sensors.is_none() {
                        status_copy.sensors = Some(api::Sensors {
                            people_now_present: vec![],
                            temperature: vec![],
                        });
                    }
                    sensor_spec
                        .template
                        .to_sensor(&value, &mut status_copy.sensors.as_mut().unwrap());
                }

                // Value could not be read, do error logging
                Err(err) => {
                    warn!(
                        "Could not retrieve key '{}' from Redis, omiting the sensor",
                        &sensor_spec.data_key
                    );
                    match err {
                        sensors::SensorError::Redis(e) => debug!("Error: {:?}", e),
                        sensors::SensorError::R2d2(e) => debug!("Error: {:?}", e),
                        sensors::SensorError::UnknownSensor(e) => warn!("Error: {:?}", e),
                    }
                }
            }
        }

        for status_modifier in &self.status_modifiers {
            status_modifier.modify(&mut status_copy);
        }

        // Serialize to JSON
        serde_json::to_string(&status_copy).expect(
            "Status object could not be serialized to JSON. \
             Please open an issue at https://github.com/spaceapi-community/spaceapi-server-rs/issues",
        )
    }
}

/*
impl middleware::Handler for ReadHandler {
    /// Return the current status JSON.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        info!("{} /{} from {}", req.method, req.url.path()[0], req.remote_addr);

        // Get response body
        let body = self.build_response_json();

        // Create response
        let response = Response::with((status::Ok, body))
            // Set headers
            .set(Header(headers::ContentType(
                "application/json; charset=utf-8".parse().unwrap(),
            )))
            .set(Header(headers::CacheControl(vec![
                headers::CacheDirective::NoCache,
            ])))
            .set(Header(headers::AccessControlAllowOrigin::Any));

        Ok(response)
    }
}
*/

pub(crate) struct UpdateHandler {
    redis_pool: RedisPool,
    sensor_specs: sensors::SafeSensorSpecs,
}

impl UpdateHandler {
    pub(crate) fn new(redis_pool: RedisPool, sensor_specs: sensors::SafeSensorSpecs) -> UpdateHandler {
        UpdateHandler {
            redis_pool,
            sensor_specs,
        }
    }

    /// Update sensor value in Redis
    fn update_sensor(&self, sensor: &str, value: &str) -> Result<(), sensors::SensorError> {
        // Validate sensor
        let sensor_spec = self
            .sensor_specs
            .iter()
            .find(|&spec| spec.data_key == sensor)
            .ok_or_else(|| sensors::SensorError::UnknownSensor(sensor.into()))?;

        // Store data
        sensor_spec.set_sensor_value(&self.redis_pool, value)
    }

    /*
    /// Build an OK response with the `HTTP 204 No Content` status code.
    fn ok_response(&self) -> Response {
        Response::with(status::NoContent)
            // Set headers
            .set(Header(headers::ContentType(
                "application/json; charset=utf-8".parse().unwrap(),
            )))
            .set(Header(headers::CacheControl(vec![
                headers::CacheDirective::NoCache,
            ])))
            .set(Header(headers::AccessControlAllowOrigin::Any))
    }

    /// Build an error response with the specified `error_code` and the specified `reason` text.
    fn err_response(&self, error_code: status::Status, reason: &str) -> Response {
        let error = ErrorResponse {
            reason: reason.into(),
        };
        let error_string = serde_json::to_string(&error).expect("Could not serialize error");
        Response::with((error_code, error_string))
            // Set headers
            .set(Header(headers::ContentType(
                "application/json; charset=utf-8".parse().unwrap(),
            )))
            .set(Header(headers::CacheControl(vec![
                headers::CacheDirective::NoCache,
            ])))
            .set(Header(headers::AccessControlAllowOrigin::Any))
    }
    */
}

/*
impl middleware::Handler for UpdateHandler {
    /// Update the sensor, return correct status code.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        // TODO: create macro for these info! invocations.
        info!("{} /{} from {}", req.method, req.url.path()[0], req.remote_addr);

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
                Some(values) => match values.len() {
                    1 => values[0].to_string(),
                    _ => return Ok(self.err_response(status::BadRequest, "Too many values specified")),
                },
                None => return Ok(self.err_response(status::BadRequest, "\"value\" parameter not specified")),
            }
        }

        // Update values in Redis
        if let Err(e) = self.update_sensor(&sensor_name, &sensor_value) {
            error!(
                "Updating sensor value for sensor \"{}\" failed: {:?}",
                &sensor_name, e
            );
            let response = match e {
                sensors::SensorError::UnknownSensor(sensor) => {
                    self.err_response(status::BadRequest, &format!("Unknown sensor: {}", sensor))
                }
                sensors::SensorError::Redis(_) | sensors::SensorError::R2d2(_) => {
                    self.err_response(status::InternalServerError, "Updating values in datastore failed")
                }
            };
            return Ok(response);
        };

        // Create response
        Ok(self.ok_response())
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_error_response() {
        let error = ErrorResponse {
            reason: "foobared".into(),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert_eq!(json, r#"{"status":"error","reason":"foobared"}"#);
    }
}
