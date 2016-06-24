//! The SpaceAPI server struct.

use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use iron::Iron;
use router::Router;
use redis::{IntoConnectionInfo, ConnectionInfo};

mod handlers;

use ::api;

use ::sensors;
use ::modifiers;
use ::errors::SpaceapiServerError;


/// A Space API server instance.
///
/// You can create a new instance using the ``new`` constructor method by
/// passing it the host, the port, the ``Status`` object and a redis connection info object.
///
/// The ``SpaceapiServer`` includes a web server through
/// [Hyper](http://hyper.rs/hyper/hyper/server/index.html). Simply call the ``serve`` method.
pub struct SpaceapiServer {
    socket_addr: SocketAddr,
    status: api::Status,
    redis_connection_info: ConnectionInfo,
    sensor_specs: sensors::SafeSensorSpecs,
    status_modifiers: Vec<Box<modifiers::StatusModifier>>,
}

impl SpaceapiServer {

    pub fn new<U, T>(socket_addr: U,
                     status: api::Status,
                     redis_connection_info: T,
                     status_modifiers: Vec<Box<modifiers::StatusModifier>>)
        -> Result<SpaceapiServer, SpaceapiServerError>
        where U: ToSocketAddrs, T: IntoConnectionInfo {
            let mut socket_addr_iter = try!(socket_addr.to_socket_addrs());
            let socket_addr = try!(socket_addr_iter.next()
                                   .ok_or(SpaceapiServerError::Message("Invalid socket address".into())));
            Ok(SpaceapiServer {
                socket_addr: socket_addr,
                status: status,
                redis_connection_info: try!(redis_connection_info.into_connection_info()),
                sensor_specs: Arc::new(Mutex::new(vec![])),
                status_modifiers: status_modifiers,
            })
        }

    /// Create and return a Router instance.
    fn route(self) -> Router {
        let mut router = Router::new();

        router.get("/", handlers::ReadHandler::new(
            self.status.clone(), self.redis_connection_info.clone(),
            self.sensor_specs.clone(), self.status_modifiers));

        router.put("/sensors/:sensor/", handlers::UpdateHandler::new(
            self.redis_connection_info.clone(), self.sensor_specs.clone()));

        router
    }

    /// Start a HTTP server listening on ``self.host:self.port``.
    ///
    /// The call returns an `HttpResult<Listening>` object, see
    /// http://ironframework.io/doc/hyper/server/struct.Listening.html
    /// for more information.
    pub fn serve(self) -> ::HttpResult<::Listening> {
        // Log some useful debug information
        debug!("Redis connection info: {:?}", &self.redis_connection_info);

        // Launch server process
        let socket_addr = self.socket_addr;
        let router = self.route();
        println!("Starting HTTP server on http://{}...", socket_addr);
        Iron::new(router).http(socket_addr)
    }

    /// Register a new sensor.
    ///
    /// The first argument is a ``api::SensorTemplate`` instance containing all static data.
    /// The second argument specifies how to get the actual sensor value from Redis.
    /// And the third argument specifies the data type of the value.
    pub fn register_sensor(&mut self, template: Box<api::SensorTemplate>, data_key: String) {
        let sensor_specs_ref = self.sensor_specs.clone();
        sensor_specs_ref.lock().unwrap().push(
            sensors::SensorSpec { template: template, data_key: data_key}
        );
    }

}
