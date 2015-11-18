//! The SpaceAPI server struct.

use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};

use iron::Iron;
use router::Router;

use ::api;
use ::api::SensorTemplate;

use ::datastore;
use ::sensors;
use ::modifiers;

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
    status_modifiers: Vec<Box<modifiers::StatusModifier>>,
}

impl SpaceapiServer {

    pub fn new(host: Ipv4Addr,
               port: u16,
               status: api::Status,
               datastore: datastore::SafeDataStore,
               status_modifiers: Vec<Box<modifiers::StatusModifier>>)
               -> SpaceapiServer {
        SpaceapiServer {
            host: host,
            port: port,
            status: status,
            datastore: datastore,
            sensor_specs: Arc::new(Mutex::new(vec![])),
            status_modifiers: status_modifiers,
        }
    }

    fn route(self) -> Router {
        router!(
            get "/" => handlers::ReadHandler::new(self.status.clone(), self.datastore.clone(), self.sensor_specs.clone(), self.status_modifiers),
            put "/sensors/:sensor/" => handlers::UpdateHandler::new(self.datastore.clone(), self.sensor_specs.clone())
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
