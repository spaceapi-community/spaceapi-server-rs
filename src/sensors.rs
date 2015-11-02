//! Sensor related stuff.

use std::sync::{Arc, Mutex};

use ::api;
use ::datastore;

/// A specification of a sensor.
///
/// The ``template`` field contains the static data of a sensor and
/// the ``data_key`` says how to find the sensor value in the datastore.
pub struct SensorSpec {
    pub template: Box<api::SensorTemplate>,
    pub data_key: String,
}

/// A vector of sensor specs, wrapped in an Arc and a Mutex. Safe for use in multithreaded situations.
/// TODO: Maybe we could use a RwLock instead of a Mutex?
pub type SafeSensorSpecs = Arc<Mutex<Vec<SensorSpec>>>;

impl SensorSpec {
    /// Retrieve sensor value from the datastore.
    pub fn get_sensor_value(&self, datastore: datastore::SafeDataStore) -> Option<String> {
        let datastore_ref = datastore.clone();
        let datastore_lock = datastore_ref.lock().unwrap();
        datastore_lock.retrieve(&self.data_key)
                      .map_err(|err| {
                          warn!("Could not retrieve key '{}' from datastore, omiting the sensor",
                                &self.data_key);
                          debug!("Error: {:?}", err);
                          err
                      })
                      .ok()
    }
}
