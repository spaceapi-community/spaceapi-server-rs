//! The SpaceAPI server struct.

use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;

use r2d2;
use r2d2_redis::RedisConnectionManager;
use iron::Iron;
use router::Router;
use redis::{IntoConnectionInfo, ConnectionInfo};

use serde_json::Value;
use serde_json::map::Map;

mod handlers;

use crate::api;

use crate::sensors;
use crate::modifiers;
use crate::errors::SpaceapiServerError;
use crate::types::RedisPool;

enum RedisInfo {
    None,
    Pool(r2d2::Pool<r2d2_redis::RedisConnectionManager>),
    ConnectionInfo(ConnectionInfo),
    Err(SpaceapiServerError),
}

pub struct SpaceapiServerBuilder {
    status: api::Status,
    redis_info: RedisInfo,
    sensor_specs: Vec<sensors::SensorSpec>,
    status_modifiers: Vec<Box<dyn modifiers::StatusModifier>>,
}

impl SpaceapiServerBuilder {

    pub fn new(mut status: api::Status) -> SpaceapiServerBuilder {
        // Instantiate versions object
        let mut versions = Map::new();
        versions.insert("spaceapi-rs".into(), api::get_version().into());
        versions.insert("spaceapi-server-rs".into(), crate::get_version().into());

        // Add to extensions
        status.extensions.insert("versions".into(), Value::Object(versions));

        SpaceapiServerBuilder {
            status,
            redis_info: RedisInfo::None,
            sensor_specs: vec![],
            status_modifiers: vec![],
        }
    }

    pub fn redis_connection_info<R: IntoConnectionInfo>(mut self, redis_connection_info: R) -> Self {
        self.redis_info = match redis_connection_info.into_connection_info() {
            Ok(ci) => RedisInfo::ConnectionInfo(ci),
            Err(e) => RedisInfo::Err(e.into()),
        };
        self
    }

    pub fn redis_pool(mut self, redis_pool: r2d2::Pool<r2d2_redis::RedisConnectionManager>) -> Self {
        self.redis_info = RedisInfo::Pool(redis_pool);
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
    pub fn add_sensor<T: api::SensorTemplate + 'static>(mut self, template: T, data_key: String) -> Self {
        self.sensor_specs.push(
            sensors::SensorSpec { template: Box::new(template), data_key }
        );
        self
    }

    pub fn build(self) -> Result<SpaceapiServer, SpaceapiServerError> {

        let pool = match self.redis_info {
            RedisInfo::None => Err("No redis connection defined".into()),
            RedisInfo::Err(e) => Err(e),
            RedisInfo::Pool(p) => Ok(p),
            RedisInfo::ConnectionInfo(ci) => {
                // Log some useful debug information
                debug!("Connecting to redis database {} at {:?}",
                       ci.db, ci.addr);

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
                let redis_manager = RedisConnectionManager::new(ci)?;
                Ok(r2d2::Pool::new(redis_config, redis_manager)?)
            }
        };

        Ok(SpaceapiServer {
            status: self.status,
            redis_pool: pool?,
            sensor_specs: Arc::new(self.sensor_specs),
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
    status: api::Status,
    redis_pool: RedisPool,
    sensor_specs: sensors::SafeSensorSpecs,
    status_modifiers: Vec<Box<dyn modifiers::StatusModifier>>,
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
    pub fn serve<S: ToSocketAddrs>(self, socket_addr: S) -> crate::HttpResult<crate::Listening> {
        // Launch server process
        let router = self.route();
        println!("Starting HTTP server on:");
        for a in socket_addr.to_socket_addrs()? {
            println!("\thttp://{}", a);
        }
        Iron::new(router).http(socket_addr)
    }
}
