//! The main entry point for the Space API server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.
#[macro_use] extern crate log;
#[macro_use] extern crate error_type;
extern crate rustc_serialize;
extern crate iron;
#[macro_use] extern crate router;
extern crate urlencoded;
extern crate redis;
extern crate spaceapi;

pub use spaceapi as api;

mod server;
mod errors;
pub mod sensors;
pub mod modifiers;

pub use server::SpaceapiServer;
pub use errors::SpaceapiServerError;
