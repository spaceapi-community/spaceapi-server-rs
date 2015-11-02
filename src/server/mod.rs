//! The SpaceAPI server struct.

use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};

use iron::Iron;
use router::Router;
use ::urlencoded;

use ::api;
use ::api::SensorTemplate;

use ::datastore;
use ::sensors;

mod handlers;


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
    status: api::Status,
    datastore: datastore::SafeDataStore,
    sensor_specs: sensors::SafeSensorSpecs,
}

impl SpaceapiServer {

    pub fn new(host: Ipv4Addr, port: u16, status: api::Status, datastore: datastore::SafeDataStore) -> SpaceapiServer {
        SpaceapiServer {
            host: host,
            port: port,
            status: status,
            datastore: datastore,
            sensor_specs: Arc::new(Mutex::new(vec![])),
        }
    }

    /// Update values in the `DataStore`
    fn update_values(&self, map: &urlencoded::QueryMap) -> Result<(), String> {
        // store data to datastore
        let datastore_ref = self.datastore.clone();
        let mut datastore_lock = datastore_ref.lock().unwrap();
        info!("{:?}", map);

        for item in map.iter() {
            // TODO: check if key exists and handle errors
            datastore_lock.store(item.0, &item.1[0]);
        }
        Ok(())
    }

    fn route(self) -> Router {
        router!(
            get "/" => handlers::ReadHandler::new(self.status.clone(), self.datastore, self.sensor_specs)
        )
    }

    /// Start a HTTP server listening on ``self.host:self.port``.
    ///
    /// This call is blocking. It can be interrupted with SIGINT (Ctrl+C).
    pub fn serve(self) {
        let host = self.host;
        let port = self.port;

        let router = self.route();

        println!("Starting HTTP server on http://{}:{}...", host, port);
        Iron::new(router).http((host, port)).unwrap();
    }

    /// Register a new sensor.
    ///
    /// The first argument is a ``api::SensorTemplate`` instance containing all static data.
    /// The second argument specifies how to get the actual sensor value from the datastore.
    /// And the third argument specifies the data type of the value.
    pub fn register_sensor(&mut self, template: Box<api::SensorTemplate>, data_key: String) {
        let sensor_specs_ref = self.sensor_specs.clone();
        sensor_specs_ref.lock().unwrap().push(
            sensors::SensorSpec { template: template, data_key: data_key}
        );
    }

}


#[cfg(test)]
mod test {
    extern crate rustc_serialize;

    use std::net::Ipv4Addr;
    use std::sync::Mutex;
    use rustc_serialize::json::Json;
    use ::api;
    use ::api::optional::Optional;
    use ::datastore::{DataStore, RedisStore};
    use super::SpaceapiServer;

    fn get_test_data() -> Json {
        // Create minimal status object
        let status = api::Status::new(
            "ourspace".to_string(),
            "https://example.com/logo.png".to_string(),
            "https://example.com/".to_string(),
            api::Location {
                address: Optional::Value("Street 1, Zürich, Switzerland".to_string()),
                lat: 47.123,
                lon: 8.88,
            },
            api::Contact {
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
        let datastore = Mutex::new(Box::new(RedisStore::new().unwrap()) as Box<DataStore>);

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
                          "logo", "space", "state", "url"]);
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
