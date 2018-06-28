//! The main entry point for the Space API server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

#![doc(html_root_url = "https://docs.rs/spaceapi-server")]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate log;
#[macro_use] extern crate error_type;
extern crate rustc_serialize;
extern crate serde_json;
extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
pub extern crate spaceapi as api;

pub use iron::error::HttpResult;
pub use iron::Listening;

mod server;
mod errors;
mod types;
pub mod sensors;
pub mod modifiers;

pub use server::SpaceapiServer;
pub use server::SpaceapiServerBuilder;
pub use errors::SpaceapiServerError;

/// Return own crate version. Used in API responses.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod test {
    use super::get_version;

    #[test]
    fn test_get_version() {
        let version = get_version();
        assert_eq!(3, version.split('.').count());
    }
}
