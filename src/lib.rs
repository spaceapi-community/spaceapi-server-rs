//! The main entry point for the Space API server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

#![doc(html_root_url = "https://coredump-ch.github.io/spaceapi-server-rs/")]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate log;
#[macro_use] extern crate error_type;
extern crate rustc_serialize;
extern crate iron;
extern crate hyper;
extern crate router;
extern crate urlencoded;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
pub extern crate spaceapi as api;

pub use iron::error::HttpResult;
pub use hyper::server::Listening;

mod server;
mod errors;
mod types;
pub mod sensors;
pub mod modifiers;

pub use server::SpaceapiServer;
pub use errors::SpaceapiServerError;
