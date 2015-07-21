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
use std::sync::Mutex;
use std::sync::Arc;

use rustc_serialize::json::{Json, ToJson};
use iron::{Request, Response, IronResult, Iron, Set};
use iron::{status, headers, middleware};
use iron::modifiers::Header;

pub use datastore::DataStore;
use spaceapi::Optional::{Value, Absent};


fn build_response_json(status: &spaceapi::Status, people_present: Option<u32>, raspi_temperature: Option<f32>) -> Json {
    let people_present_sensor = match people_present {
        Some(count) => Value(vec![
            spaceapi::PeopleNowPresentSensor {
                value: count,
                location: Value("Hackerspace".to_string()),
                name: Absent,
                description: Absent,
                names: Absent,
            }
        ]),
        None => Absent,
    };

    let temperature_sensor = match raspi_temperature {
        Some(degrees) => Value(vec![
            spaceapi::TemperatureSensor {
                value: degrees,
                unit: "°C".to_string(),
                location: "Basement".to_string(),
                name: Value("Raspberry CPU".to_string()),
                description: Absent,
            }
        ]),
        None => Absent,
    };

    // Create a mutable copy of the status struct and add sensor data.
    let mut status_copy = (*status).clone();
    status_copy.sensors = Value(spaceapi::Sensors {
        people_now_present: people_present_sensor,
        temperature: temperature_sensor,
    });

    // Serialize to JSON
    status_copy.to_json()
}


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
    datastore: Arc<Mutex<Box<DataStore>>>,
}

impl SpaceapiServer {

    pub fn new(host: Ipv4Addr, port: u16, status: spaceapi::Status, datastore: Arc<Mutex<Box<DataStore>>>) -> SpaceapiServer{
        SpaceapiServer {
            host: host,
            port: port,
            status: status,
            datastore: datastore,
        }
    }

    pub fn serve(self) {
        let host = self.host;
        let port = self.port;
        println!("Starting HTTP server on {}:{}...", host, port);
        Iron::new(self).http((host, port)).unwrap();
    }

}

impl middleware::Handler for SpaceapiServer {

    fn handle(&self, _: &mut Request) -> IronResult<Response> {

        // Fetch data from datastore
        let datastore_clone = self.datastore.clone();
        let datastore_lock = datastore_clone.lock().unwrap();
        let people_present: Option<u32> = match datastore_lock.retrieve("people_present") {
            Ok(v) => match v.parse::<u32>() {
                Ok(i) => Some(i),
                Err(_) => None,
            },
            Err(_) => None,
        };
        let raspi_temperature: Option<f32> = match datastore_lock.retrieve("raspi_temperature") {
            Ok(v) => match v.parse::<f32>() {
                Ok(i) => Some(i),
                Err(_) => None,
            },
            Err(_) => None,
        };

        // Get response body
        let body = build_response_json(&self.status, people_present, raspi_temperature).to_string();

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

    use super::build_response_json;
    use spaceapi::Optional;
    use rustc_serialize::json::Json;

    fn get_test_data() -> Json {
        // Create minimal status object
        let ref status = spaceapi::Status::new(
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

        // Add sensor data, build JSON
        let people_present = Some(23);
        let temperature = Some(42.5);
        build_response_json(status, people_present, temperature)
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

}
