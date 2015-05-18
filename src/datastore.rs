extern crate redis;

/// A DataStore needs to implement store and retrieve methods.
pub trait DataStore {
    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError>;
    fn retrieve(&self, key: &str) -> Result<String, DataStoreError>;
    fn delete(&self, key: &str) -> Result<(), DataStoreError>;
}

/// A struct representing a datastore error.
#[derive(Debug)]
pub struct DataStoreError {
    pub repr: ErrorKind,
}

/// An enum containing all possible error kinds.
#[derive(Debug)]
pub enum ErrorKind {
    RedisError(redis::RedisError),
}

impl From<redis::RedisError> for DataStoreError {
    fn from(err: redis::RedisError) -> DataStoreError {
        DataStoreError { repr: ErrorKind::RedisError(err) }
    }
}
