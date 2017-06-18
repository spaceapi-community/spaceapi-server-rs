//! Sensor related stuff.

use std::sync::Arc;

use r2d2;
use redis::Commands;
use redis::RedisError;

use ::api;
use ::types::RedisPool;

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
        },
        R2d2(r2d2::GetTimeout) {
            cause;
        }
    }
}

/// A vector of sensor specs, wrapped in an Arc. Safe for use in multithreaded situations.
pub type SafeSensorSpecs = Arc<Vec<SensorSpec>>;

impl SensorSpec {

    /// Retrieve sensor value from Redis.
    pub fn get_sensor_value(&self, redis_pool: RedisPool) -> Result<String, SensorError> {
        let conn = try!(redis_pool.get());
        let value: String = try!(conn.get(&*self.data_key));
        Ok(value)
    }

    /// Set sensor value in Redis.
    pub fn set_sensor_value(&self, redis_pool: RedisPool, value: &str) -> Result<(), SensorError> {
        let conn = try!(redis_pool.get());
        let _: String = try!(conn.set(&*self.data_key, value));
        Ok(())
    }

}
