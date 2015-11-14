extern crate redis;

use std::env;

use self::redis::{Client, Commands, ConnectionInfo, ConnectionAddr};

use super::{DataStore, DataStoreError};


/// A data store for Redis.
pub struct RedisStore {
    client: Client,
}

/// Implement the DataStore methods for Redis
impl DataStore for RedisStore {

    fn store(&mut self, key: &str, value: &str) -> Result<(), DataStoreError> {
        let con = try!(self.client.get_connection());

        try!(con.set(key, value));
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<String, DataStoreError> {
        let con = try!(self.client.get_connection());

        Ok(try!(con.get(key)))
    }

    fn delete(&mut self, key: &str) -> Result<(), DataStoreError> {
        let con = try!(self.client.get_connection());

        Ok(try!(con.del(key)))
    }

}

impl RedisStore {
    pub fn new() -> Result<RedisStore, DataStoreError> {
        // Read env variables
        let redis_host: String = env::var("REDIS_HOST").unwrap_or("127.0.0.1".to_string());
        let redis_port: u16 = env::var("REDIS_PORT").unwrap_or("6379".to_string()).parse().unwrap_or(6379);
        let redis_db: i64 = env::var("REDIS_DB").unwrap_or("0".to_string()).parse().unwrap_or(0);

        // Build connection info
        info!("Connecting to redis://{}:{}/{}...", &redis_host, &redis_port, &redis_db);
        let connection_info = ConnectionInfo {
            addr: Box::new(ConnectionAddr::Tcp(redis_host, redis_port)),
            db: redis_db,
            passwd: None,
        };

        // Create redis client
        let redis_client = try!(Client::open(connection_info));

        Ok(RedisStore { client: redis_client })
    }
}

#[cfg(test)]
mod test {
    use datastore::DataStore;
    use super::RedisStore;

    #[test]
    fn roundtrip() {
        let mut rs = RedisStore::new().unwrap();
        rs.store("key", "value").unwrap();
        let result = rs.retrieve("key").unwrap();
        assert_eq!(result, "value");
        rs.delete("key").unwrap();
    }

    #[test]
    #[should_panic(expected = "response was nil")]
    fn nonexistant() {
        let rs = RedisStore::new().unwrap();
        rs.retrieve("nonexistant").unwrap();
    }

}
