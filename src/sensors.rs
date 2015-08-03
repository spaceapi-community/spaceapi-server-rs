//! Sensor related stuff.

use super::api;
use super::datastore::SafeDataStore;

/// A specification of a sensor.
///
/// The ``template`` field contains the static data of a sensor and
/// the ``data_key`` says how to find the sensor value in the datastore.
#[derive(Debug)]
pub struct SensorSpec {
    pub template: api::SensorTemplate,
    pub data_key: String,
    pub data_type: SensorValueType,
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

impl SensorSpec {

    /// Retrieve sensor value from the datastore.
    pub fn get_sensor_value(&self, datastore: &SafeDataStore) -> Option<SensorValue> {
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
