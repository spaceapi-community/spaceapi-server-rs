//! The main entry point for the Space API server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

extern crate rustc_serialize;
extern crate iron;
extern crate spaceapi;

pub mod datastore;
pub mod redis_store;

use std::net::Ipv4Addr;
use std::sync::{Mutex, Arc};

use rustc_serialize::json::{Json, ToJson};
use iron::{Request, Response, IronResult, Iron, Set};
use iron::{status, headers, middleware};
use iron::modifiers::Header;

pub use datastore::DataStore;
use spaceapi::utils::Optional;
use spaceapi::SensorTemplate::{PeopleNowPresentSensorTemplate, TemperatureSensorTemplate};


/// A specification of a sensor.
///
/// The ``template`` field contains the static data of a sensor and
/// the ``data_key`` says how to find the sensor value in the datastore.
#[derive(Debug)]
pub struct SensorSpec {
    template: spaceapi::SensorTemplate,
    data_key: String,
    data_type: SensorValueType,
}

/// All possible value types a sensor can have.
#[derive(Debug)]
pub enum SensorValueType {
    Int,
    Float,
    Bool,
}

/// The actual sensor values.
#[derive(Debug)]
pub enum SensorValue {
    Int(i64),
    Float(f64),
    Bool(bool),
}

/// A datastore wrapped in an Arc, a Mutex and a Box. Safe for use in multithreaded situations.
pub type SafeDataStore = Arc<Mutex<Box<DataStore>>>;

/// A Space API server instance.
///
/// You can create a new instance using the ``new`` constructor method by
/// passing it the host, the port, the ``Status`` object and a datastore.
///
/// The ``SpaceapiServer`` includes a web server through
/// [Hyper](http://hyper.rs/hyper/hyper/server/index.html). Simply call the ``serve`` method.
pub struct SpaceapiServer {
    host: Ipv4Addr,
    port: u16,
    status: spaceapi::Status,
    datastore: SafeDataStore,
    sensor_specs: Vec<SensorSpec>
}

impl SpaceapiServer {

    pub fn new(host: Ipv4Addr, port: u16, status: spaceapi::Status, datastore: SafeDataStore) -> SpaceapiServer {
        SpaceapiServer {
            host: host,
            port: port,
            status: status,
            datastore: datastore,
            sensor_specs: vec![],
        }
    }

    /// Start a HTTP server listening on ``self.host:self.port``.
    ///
    /// This call is blocking. It can be interrupted with SIGINT (Ctrl+C).
    pub fn serve(self) {
        let host = self.host;
        let port = self.port;
        println!("Starting HTTP server on http://{}:{}...", host, port);
        Iron::new(self).http((host, port)).unwrap();
    }

    /// Register a new sensor.
    ///
    /// The first argument is a ``spaceapi::SensorTemplate`` instance containing all static data.
    /// The second argument specifies how to get the actual sensor value from the datastore.
    /// And the third argument specifies the data type of the value.
    pub fn register_sensor(&mut self, template: spaceapi::SensorTemplate, data_key: String, data_type: SensorValueType) {
        self.sensor_specs.push(
            SensorSpec { template: template, data_key: data_key, data_type: data_type }
        );
    }

    fn build_response_json(&self) -> Json {

        // Create a mutable copy of the status struct
        let mut status_copy = self.status.clone();

        // Prepare sensor vector
        let mut temperature_sensors: Vec<spaceapi::TemperatureSensor> = Vec::new();
        let mut people_now_present_sensors: Vec<spaceapi::PeopleNowPresentSensor> = Vec::new();

        // Process registered sensors
        if !self.sensor_specs.is_empty() {

            // Add sensor data
            for sensor_spec in &self.sensor_specs {

                let value = sensor_spec.get_sensor_value(&self.datastore);

                // If value is available, save sensor data
                if value.is_some() {
                    match sensor_spec.template {

                        // Create temperature sensor instance
                        TemperatureSensorTemplate { ref unit, ref location, ref name, ref description } => {
                            let sensor = spaceapi::TemperatureSensor {
                                unit: (*unit).clone(),
                                location: (*location).clone(),
                                name: (*name).clone(),
                                description: (*description).clone(),
                                value: match value.unwrap() {
                                    SensorValue::Float(v) => v,
                                    _ => unreachable!(),
                                },
                            };
                            temperature_sensors.push(sensor);
                        },

                        // Create people now present sensor instance
                        PeopleNowPresentSensorTemplate { ref location, ref name, ref names, ref description } => {
                            let sensor = spaceapi::PeopleNowPresentSensor {
                                location: (*location).clone(),
                                name: (*name).clone(),
                                names: (*names).clone(),
                                description: (*description).clone(),
                                value: match value.unwrap() {
                                    SensorValue::Int(v) => v,
                                    _ => unreachable!(),
                                },
                            };
                            people_now_present_sensors.push(sensor);
                        },

                    }
                }

            }

        }

        // Add sensors to status object
        status_copy.sensors = Optional::Value(spaceapi::Sensors {
            people_now_present: match people_now_present_sensors.is_empty() {
                true => Optional::Absent,
                false => Optional::Value(people_now_present_sensors),
            },
            temperature: match temperature_sensors.is_empty() {
                true => Optional::Absent,
                false => Optional::Value(temperature_sensors),
            },
        });

        // Serialize to JSON
        status_copy.to_json()
    }

}

impl SensorSpec {

    /// Retrieve sensor value from the datastore.
    fn get_sensor_value(&self, datastore: &SafeDataStore) -> Option<SensorValue> {
        let datastore_clone = datastore.clone();
        let datastore_lock = datastore_clone.lock().unwrap();
        match datastore_lock.retrieve(&self.data_key) {
            Ok(v) => {
                match self.data_type {
                    SensorValueType::Float => match v.parse::<f64>() {
                        Ok(i) => Some(SensorValue::Float(i)),
                        Err(_) => None,
                    },
                    SensorValueType::Int => match v.parse::<i64>() {
                        Ok(i) => Some(SensorValue::Int(i)),
                        Err(_) => None,
                    },
                    SensorValueType::Bool => None,  // TODO
                }
            },
            Err(_) => None,
        }
    }

}

impl middleware::Handler for SpaceapiServer {

    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        // Get response body
        let body = self.build_response_json().to_string();

        // Create response
        let mut response = Response::with((status::Ok, body));

        // Set headers
        response.set_mut(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())));
        response.set_mut(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])));
        response.set_mut(Header(headers::AccessControlAllowOrigin::Any));

        Ok(response)
    }
}


#[cfg(test)]
mod test {
    extern crate spaceapi;
    extern crate rustc_serialize;

    use std::net::Ipv4Addr;
    use std::sync::{Mutex,Arc};
    use rustc_serialize::json::Json;
    use spaceapi::utils::Optional;
    use super::SpaceapiServer;
    use super::datastore::DataStore;
    use super::redis_store::RedisStore;

    fn get_test_data() -> Json {
        // Create minimal status object
        let status = spaceapi::Status::new(
            "ourspace".to_string(),
            "https://example.com/logo.png".to_string(),
            "https://example.com/".to_string(),
            spaceapi::Location {
                address: Optional::Value("Street 1, Zürich, Switzerland".to_string()),
                lat: 47.123,
                lon: 8.88,
            },
            spaceapi::Contact {
                irc: Optional::Absent,
                twitter: Optional::Absent,
                foursquare: Optional::Absent,
                email: Optional::Value("hi@example.com".to_string()),
            },
            vec![
                "email".to_string(),
                "twitter".to_string(),
            ],
        );

        // Create datastore (TODO: Create dummy store for testing?)
        let datastore = Arc::new(Mutex::new(Box::new(RedisStore::new().unwrap()) as Box<DataStore>));

        // Initialize server
        let server = SpaceapiServer::new(Ipv4Addr::new(127, 0, 0, 1), 3001, status, datastore);

        // Build JSON
        server.build_response_json()
    }

    #[test]
    /// Verify that the result is a JSON object.
    fn verify_json_obj() {
        let json = get_test_data();
        assert!(json.is_object());
    }

    #[test]
    /// Verify that the result has the correct keys.
    fn verify_json_keys() {
        let json = get_test_data();
        let status = json.as_object().unwrap();  // We get back a BTreeMap<String, Json>
        let keys: Vec<String> = status.keys().cloned().collect();  // Collect the keys
        assert_eq!(keys, ["api", "contact", "issue_report_channels", "location",
                          "logo", "sensors", "space", "state", "url"]);
    }

    #[test]
    /// Verify static data
    fn verify_json_static_data() {
        let json = get_test_data();
        let status = json.as_object().unwrap();  // We get back a BTreeMap<String, Json>

        // Strings
        assert_eq!(status.get("api").unwrap().as_string().unwrap(), "0.13");
        assert_eq!(status.get("space").unwrap().as_string().unwrap(), "ourspace");
        assert_eq!(status.get("url").unwrap().as_string().unwrap(), "https://example.com/");
        assert_eq!(status.get("logo").unwrap().as_string().unwrap(), "https://example.com/logo.png");

        // Channels
        let channels: &Vec<Json> = status.get("issue_report_channels").unwrap().as_array().unwrap();
        let channel_values: Vec<String> = channels.iter()
                                                  .cloned()
                                                  .map(|c| c.as_string().unwrap().to_string())
                                                  .collect();
        assert_eq!(vec!["email", "twitter"], channel_values);
    }

    #[test]
    /// Verify location
    fn verify_json_location() {
        let json = get_test_data();
        let status = json.as_object().unwrap();
        let location = status.get("location").unwrap().as_object().unwrap();

        // Verify data
        let address = location.get("address").unwrap().as_string().unwrap();
        assert_eq!("Street 1, Zürich, Switzerland".to_string(), address);
        assert_eq!(47.123, location.get("lat").unwrap().as_f64().unwrap());
        assert_eq!(8.88, location.get("lon").unwrap().as_f64().unwrap());
    }

    #[test]
    /// Verify contact
    fn verify_json_contact() {
        let json = get_test_data();
        let status = json.as_object().unwrap();
        let contact = status.get("contact").unwrap().as_object().unwrap();

        // Verify data
        let email = contact.get("email").unwrap().as_string().unwrap();
        assert_eq!("hi@example.com".to_string(), email);
    }

    /* TODO: Update
    #[test]
    /// Verify sensor data
    fn verify_json_sensors() {
        let json = get_test_data();
        let status = json.as_object().unwrap();
        let sensors = status.get("sensors").unwrap().as_object().unwrap();
        let temperature_sensor = sensors.get("temperature").unwrap()
                                        .as_array().unwrap()[0]  // List of sensors
                                        .as_object().unwrap();  // Sensor object
        let people_sensor = sensors.get("people_now_present").unwrap()  // List of sensors
                                        .as_array().unwrap()[0]  // Sensor object
                                        .as_object().unwrap();

        // Verify temperature sensor
        let temp_location = temperature_sensor.get("location").unwrap().as_string().unwrap();
        let temp_name = temperature_sensor.get("name").unwrap().as_string().unwrap();
        let temp_unit = temperature_sensor.get("unit").unwrap().as_string().unwrap();
        let temp_value = temperature_sensor.get("value").unwrap().as_f64().unwrap();
        assert_eq!("Basement".to_string(), temp_location);
        assert_eq!("Raspberry CPU".to_string(), temp_name);
        assert_eq!("°C".to_string(), temp_unit);
        assert_eq!(42.5, temp_value);

        // Verify peoplesensor
        let people_location = people_sensor.get("location").unwrap().as_string().unwrap();
        let people_value = people_sensor.get("value").unwrap().as_u64().unwrap();
        assert_eq!("Hackerspace".to_string(), people_location);
        assert_eq!(23, people_value);
    }
    */

}
