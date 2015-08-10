extern crate redis;

use std::sync::{Mutex,Arc};

#[doc(no_inline)]
pub use self::redis::RedisError;


/// A ``DataStore`` needs to implement ``store`` and ``retrieve`` methods.
pub trait DataStore : Send {
    fn store(&mut self, key: &str, value: &str) -> Result<(), DataStoreError>;
    fn retrieve(& self, key: &str) -> Result<String, DataStoreError>;
    fn delete(&mut self, key: &str) -> Result<(), DataStoreError>;
}

/// A datastore wrapped in an Arc, a Mutex and a Box. Safe for use in multithreaded situations.
pub type SafeDataStore = Arc<Mutex<Box<DataStore>>>;

/// An enum representing a datastore error.
#[derive(Debug)]
pub enum DataStoreError {
    RedisError(redis::RedisError),
    HashMapError,
}

impl From<redis::RedisError> for DataStoreError {
    fn from(err: redis::RedisError) -> DataStoreError {
        DataStoreError::RedisError(err)
    }
}
