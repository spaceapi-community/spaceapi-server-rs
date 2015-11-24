//! Sensor related stuff.

use std::sync::{Arc, Mutex};

use redis::{ConnectionInfo, Client, Connection, Commands};
use redis::{RedisResult, RedisError};

use ::api;

/// A specification of a sensor.
///
/// The ``template`` field contains the static data of a sensor and
/// the ``data_key`` says how to find the sensor value in Redis.
pub struct SensorSpec {
    pub template: Box<api::SensorTemplate>,
    pub data_key: String,
}

error_type! {
    /// A ``SensorError`` wraps problems that can occur when reading or updating sensor values.
    ///
    /// This type is only used for internal purposes and should not be used by third party code.
    #[derive(Debug)]
    pub enum SensorError {
        UnknownSensor(String) {
            desc (sensor) &sensor;
        },
        Redis(RedisError) {
            cause;
        }
    }
}

/// A vector of sensor specs, wrapped in an Arc and a Mutex. Safe for use in multithreaded situations.
/// TODO: Maybe we could use a RwLock instead of a Mutex?
pub type SafeSensorSpecs = Arc<Mutex<Vec<SensorSpec>>>;

impl SensorSpec {

    /// Return the Redis connection.
    ///
    /// TODO: Clones the `ConnectionInfo` for now, see https://github.com/mitsuhiko/redis-rs/issues/74
    fn get_redis_connection(&self, redis_connection_info: &ConnectionInfo) -> RedisResult<Connection> {
        let client = try!(Client::open(redis_connection_info.clone()));
        client.get_connection()
    }

    /// Retrieve sensor value from Redis.
    pub fn get_sensor_value(&self, redis_connection_info: &ConnectionInfo) -> Result<String, SensorError> {
        let conn = try!(self.get_redis_connection(redis_connection_info));
        let value: String = try!(conn.get(&*self.data_key));
        Ok(value)
    }

    /// Set sensor value in Redis.
    pub fn set_sensor_value(&self, redis_connection_info: &ConnectionInfo, value: &str) -> Result<(), SensorError> {
        let conn = try!(self.get_redis_connection(redis_connection_info));
        let _: String = try!(conn.set(&*self.data_key, value));
        Ok(())
    }

}
