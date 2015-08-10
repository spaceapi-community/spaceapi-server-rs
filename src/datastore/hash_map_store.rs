use super::{DataStore, DataStoreError};
use std::collections::HashMap;

/// Implement the DataStore methods for HashMap
impl DataStore for HashMap<String,String> {

    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError> {
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<String, DataStoreError> {
        Err(DataStoreError::HashMapError)
    }

    fn delete(&self, key: &str) -> Result<(), DataStoreError> {
        Ok(())
    }

}

