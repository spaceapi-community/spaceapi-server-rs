use super::{DataStore, DataStoreError};
use std::collections::HashMap;

pub type HashMapStore = HashMap<String,String>;

/// Implement the DataStore methods for HashMap
impl DataStore for HashMapStore {

    fn store(&mut self, key: &str, value: &str) -> Result<(), DataStoreError> {
        self.insert(key.into(), value.into());
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<String, DataStoreError> {
        self.get(key).map(|v| v.clone()).ok_or(DataStoreError::HashMapError)
    }

    fn delete(&mut self, key: &str) -> Result<(), DataStoreError> {
        self.remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use datastore::DataStore;
    use super::*;

    #[test]
    fn roundtrip() {
        let mut store = HashMapStore::new();
        store.store("key", "value").unwrap();
        let result = store.retrieve("key").unwrap();
        assert_eq!(result, "value");
        store.delete("key").unwrap();
    }

    #[test]
    #[should_panic()]
    fn nonexistant() {
        let store = HashMapStore::new();
        store.retrieve("nonexistant").unwrap();
    }

}
