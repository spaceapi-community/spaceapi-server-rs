//! The main entry point for the Space API server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

#![doc(html_root_url = "https://docs.rs/spaceapi-server")]

pub use spaceapi as api;

pub use iron::error::HttpResult;
pub use iron::Listening;

mod server;
mod errors;
mod types;
pub mod sensors;
pub mod modifiers;

pub use crate::server::SpaceapiServer;
pub use crate::server::SpaceapiServerBuilder;
pub use crate::errors::SpaceapiServerError;

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
