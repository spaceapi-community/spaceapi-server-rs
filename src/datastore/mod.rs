//! Data store related stuff.
//!
//! Please don't import from the `common` module, use the re-exports instead.

// Regarding the "pub mod", see http://stackoverflow.com/q/30224795/284318
// and https://github.com/rust-lang/rust/issues/16264

pub mod common;
mod redis_store;
mod hash_map_store;

#[doc(inline)]
pub use self::common::{DataStore, DataStoreError, SafeDataStore};
pub use self::redis_store::RedisStore;
