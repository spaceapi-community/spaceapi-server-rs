extern crate redis;

#[doc(no_inline)]
pub use self::redis::{RedisError};


/// A ``DataStore`` needs to implement ``store`` and ``retrieve`` methods.
pub trait DataStore : Send {
    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError>;
    fn retrieve(&self, key: &str) -> Result<String, DataStoreError>;
    fn delete(&self, key: &str) -> Result<(), DataStoreError>;
}

/// An enum representing a datastore error.
#[derive(Debug)]
pub enum DataStoreError {
    RedisError(redis::RedisError),
}

impl From<redis::RedisError> for DataStoreError {
    fn from(err: redis::RedisError) -> DataStoreError {
        DataStoreError::RedisError(err)
    }
}

