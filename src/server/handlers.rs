//! Handlers for the server.

use rustc_serialize::json::{Json, ToJson};
use iron::{status, headers, middleware};
use iron::IronResult;
use iron::prelude::Set;
use iron::request::Request;
use iron::response::Response;
use iron::modifiers::Header;

use ::api;
use ::api::optional::Optional;
use ::datastore;
use ::sensors;


pub struct ReadHandler {
    status: api::Status,
    datastore: datastore::SafeDataStore,
    sensor_specs: sensors::SafeSensorSpecs,
}

impl ReadHandler {
    pub fn new(status: api::Status, datastore: datastore::SafeDataStore, sensor_specs: sensors::SafeSensorSpecs) -> ReadHandler {
        ReadHandler {
            status: status,
            datastore: datastore,
            sensor_specs: sensor_specs,
        }
    }

    fn build_response_json(&self) -> Json {

        // Create a mutable copy of the status struct
        let mut status_copy = self.status.clone();

        // Process registered sensors
        let sensor_specs_ref = self.sensor_specs.clone();
        for sensor_spec in sensor_specs_ref.lock().unwrap().iter() {
            sensor_spec.get_sensor_value(self.datastore.clone()).map(|value| {
                if status_copy.sensors.is_absent() {
                    status_copy.sensors = Optional::Value(api::Sensors {
                        people_now_present: Optional::Absent,
                        temperature: Optional::Absent,
                    });
                }
                sensor_spec.template.to_sensor(&value, &mut status_copy.sensors.as_mut().unwrap());
            });
        }

        status_copy.state.open = status_copy.sensors.as_ref()
            .and_then(|sensors| sensors.people_now_present.as_ref())
            .and_then(|people_now_present| {
                match people_now_present[0].value {
                    0i64 => Optional::Value(false),
                    _ => Optional::Value(true),
                }
            }).into();

        // Serialize to JSON
        status_copy.to_json()
    }
}

impl middleware::Handler for ReadHandler {

    /// Return the current status JSON.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        println!("{} /{} from {}", req.method, req.url.path[0], req.remote_addr);

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
