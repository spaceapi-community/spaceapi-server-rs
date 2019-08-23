//! Sensor related stuff.

use std::sync::Arc;

use quick_error::quick_error;
use r2d2;
use redis::Commands;
use redis::RedisError;

use crate::api;
use crate::types::RedisPool;

/// A specification of a sensor.
///
/// The ``template`` field contains the static data of a sensor and
/// the ``data_key`` says how to find the sensor value in Redis.
pub(crate) struct SensorSpec {
    /// A reference to an instantiated sensor template
    pub(crate) template: Box<dyn api::SensorTemplate>,
    /// The data key that is used to store and update the sensor value
    pub(crate) data_key: String,
}

quick_error! {
    /// A ``SensorError`` wraps problems that can occur when reading or updating sensor values.
    ///
    /// This type is only used for internal purposes and should not be used by third party code.
    ///
    /// TODO: Make pub(crate) when https://github.com/tailhook/quick-error/issues/48 is fixed
    #[derive(Debug)]
    pub enum SensorError {
        /// Sensor `data_key` not known
        UnknownSensor(err: String) {
            description(err)
        }
        /// Redis error
        Redis(err: RedisError) {
            from()
            cause(err)
        }
        /// R2d2 connection pool error
        R2d2(err: r2d2::GetTimeout) {
            from()
            cause(err)
        }
    }
}

/// A vector of sensor specs, wrapped in an Arc. Safe for use in multithreaded situations.
pub(crate) type SafeSensorSpecs = Arc<Vec<SensorSpec>>;

impl SensorSpec {
    /// Retrieve sensor value from Redis.
    pub(crate) fn get_sensor_value(&self, redis_pool: &RedisPool) -> Result<String, SensorError> {
        let conn = redis_pool.get()?;
        let value: String = conn.get(&*self.data_key)?;
        Ok(value)
    }

    /// Set sensor value in Redis.
    pub(crate) fn set_sensor_value(&self, redis_pool: &RedisPool, value: &str) -> Result<(), SensorError> {
        let conn = redis_pool.get()?;
        let _: String = conn.set(&*self.data_key, value)?;
        Ok(())
    }
}
