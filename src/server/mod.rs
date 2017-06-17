//! The SpaceAPI server struct.

use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use r2d2;
use r2d2_redis::RedisConnectionManager;
use iron::Iron;
use router::Router;
use redis::{IntoConnectionInfo, ConnectionInfo};

mod handlers;

use ::api;

use ::sensors;
use ::modifiers;
use ::errors::SpaceapiServerError;
use ::types::RedisPool;

pub struct SpaceapiServerBuilder {
    status: api::Status,
    socket_addr: Result<SocketAddr, SpaceapiServerError>,
    redis_connection_info: Result<ConnectionInfo, SpaceapiServerError>,
    sensor_specs: sensors::SafeSensorSpecs,
    status_modifiers: Vec<Box<modifiers::StatusModifier>>,
}

impl SpaceapiServerBuilder {

    pub fn new(status: api::Status) -> SpaceapiServerBuilder {
        SpaceapiServerBuilder {
            status: status,
            socket_addr: Err("socket_addr missing".into()),
            redis_connection_info: Err("redis_connection_info missing".into()),
            sensor_specs: Arc::new(Mutex::new(vec![])),
            status_modifiers: vec![],
        }
    }

    pub fn socket_addr<S: ToSocketAddrs>(mut self, socket_addr: S) -> Self {
        self.socket_addr = socket_addr.to_socket_addrs()
            .map_err(|e| e.into())
            .and_then(|mut i| i.next().ok_or("Invalid socket address".into()));
        self
    }

    pub fn redis_connection_info<R: IntoConnectionInfo>(mut self, redis_connection_info: R) -> Self {
        self.redis_connection_info = redis_connection_info.into_connection_info().map_err(|e| e.into());
        self
    }

    pub fn add_status_modifier<M: modifiers::StatusModifier + 'static>(mut self, modifier: M) -> Self {
        self.status_modifiers.push(Box::new(modifier));
        self
    }

    /// Add a new sensor.
    ///
    /// The first argument is a ``api::SensorTemplate`` instance containing all static data.
    /// The second argument specifies how to get the actual sensor value from Redis.
    pub fn add_sensor(self, template: Box<api::SensorTemplate>, data_key: String) -> Self {
        let sensor_specs_ref = self.sensor_specs.clone();
        sensor_specs_ref.lock().unwrap().push(
            sensors::SensorSpec { template: template, data_key: data_key}
        );
        self
    }

    pub fn build(self) -> Result<SpaceapiServer, SpaceapiServerError> {
        // Log some useful debug information
        debug!("Redis connection info: {:?}", &self.redis_connection_info);

        // Create redis pool
        let redis_config = r2d2::Config::builder()
            // Provide up to 6 connections in connection pool
            .pool_size(6)
            // At least 1 connection must be active
            .min_idle(Some(2))
            // Initialize connection pool lazily. This allows the SpaceAPI
            // server to work even without a database connection.
            .initialization_fail_fast(false)
            // Try to get a connection for max 1 second
            .connection_timeout(Duration::from_secs(1))
            // Don't log errors directly.
            // They can get quite verbose, and we're already catching and
            // logging the corresponding results anyways.
            .error_handler(Box::new(r2d2::NopErrorHandler))
            .build();
        let redis_manager = try!(RedisConnectionManager::new(self.redis_connection_info?));
        let pool = try!(r2d2::Pool::new(redis_config, redis_manager));

        Ok(SpaceapiServer {
            socket_addr: self.socket_addr?,
            status: self.status,
            redis_pool: pool,
            sensor_specs: self.sensor_specs,
            status_modifiers: self.status_modifiers,
        })
    }
}


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
    redis_pool: RedisPool,
    sensor_specs: sensors::SafeSensorSpecs,
    status_modifiers: Vec<Box<modifiers::StatusModifier>>,
}

impl SpaceapiServer {

    /// Create and return a Router instance.
    fn route(self) -> Router {
        let mut router = Router::new();

        router.get("/", handlers::ReadHandler::new(
            self.status.clone(), self.redis_pool.clone(),
            self.sensor_specs.clone(), self.status_modifiers), "root");

        router.put("/sensors/:sensor/", handlers::UpdateHandler::new(
            self.redis_pool.clone(), self.sensor_specs.clone()), "sensors");

        router
    }

    /// Start a HTTP server listening on ``self.host:self.port``.
    ///
    /// The call returns an `HttpResult<Listening>` object, see
    /// http://ironframework.io/doc/hyper/server/struct.Listening.html
    /// for more information.
    pub fn serve(self) -> ::HttpResult<::Listening> {
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
    pub fn register_sensor(&mut self, template: Box<api::SensorTemplate>, data_key: String) {
        let sensor_specs_ref = self.sensor_specs.clone();
        sensor_specs_ref.lock().unwrap().push(
            sensors::SensorSpec { template: template, data_key: data_key}
        );
    }

}
