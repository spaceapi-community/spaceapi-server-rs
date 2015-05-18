extern crate redis;

use self::redis::{Client, Commands};

use datastore::{DataStore, DataStoreError};


/// A data store for Redis.
pub struct RedisStore {
    client : Client,
}

impl RedisStore {
    pub fn new() -> Result<RedisStore, DataStoreError> {
        let redis_client = try!(Client::open("redis://127.0.0.1/"));
        Ok(RedisStore { client: redis_client })
    }
}

/// Implement the DataStore methods for Redis
impl DataStore for RedisStore {

    fn store(&self, key: &str, value: &str) -> Result<(), DataStoreError> {
        let con = try!(self.client.get_connection());

        try!(con.set(key, value));
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<String, DataStoreError> {
        let con = try!(self.client.get_connection());

        Ok(try!(con.get(key)))
    }

    fn delete(&self, key: &str) -> Result<(), DataStoreError> {
        let con = try!(self.client.get_connection());

        Ok(try!(con.del(key)))
    }

}

#[cfg(test)]
mod test {
    use redis_store;
    use datastore::DataStore;

    #[test]
    fn roundtrip() {
        let rs = redis_store::RedisStore::new().unwrap();
        rs.store("key", "value");
        let result = rs.retrieve("key").unwrap();
        assert_eq!(result, "value");
        rs.delete("key");
    }

    #[test]
    #[should_panic(expected = "response was nil")]
    fn nonexistant() {
        let rs = redis_store::RedisStore::new().unwrap();
        rs.retrieve("nonexistant").unwrap();
    }

}
