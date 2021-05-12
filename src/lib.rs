//! This crate enables you to create your own SpaceAPI server endpoint using
//! Rust. In the end you'll get a single binary that you can run on your
//! server.
//!
//!
//! ## Requirements
//!
//! On the build machine:
//!
//! - Rust and Cargo ([https://rustup.rs/](https://rustup.rs/))
//!
//! On the server:
//!
//! - Redis
//!
//! The Redis instance will be used to store dynamic data like sensor values,
//! as well as keys for dynamic data update authentication.
//!
//!
//! ## Getting Started
//!
//! Create a new Rust project:
//!
//! ```text
//! cargo new --bin mystatus
//! ```
//!
//! Add the `spaceapi-server` dependency to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! spaceapi-server = "0.4"
//! ```
//!
//! Create a `main.rs`:
//!
//! ```no_run
//! use spaceapi_server::api::{Contact, Location, StatusBuilder};
//! use spaceapi_server::SpaceapiServerBuilder;
//!
//! fn main() {
//!     // Create new minimal v14 Status instance
//!     let status = StatusBuilder::v14("coredump")
//!         .logo("https://www.coredump.ch/logo.png")
//!         .url("https://www.coredump.ch/")
//!         .location(Location {
//!             address: Some("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".into()),
//!             lat: 47.22936,
//!             lon: 8.82949,
//!         })
//!         .contact(Contact {
//!             irc: Some("irc://freenode.net/#coredump".into()),
//!             twitter: Some("@coredump_ch".into()),
//!             ..Default::default()
//!         })
//!         .build()
//!         .expect("Creating status failed");
//!
//!     // Set up server
//!     let server = SpaceapiServerBuilder::new(status)
//!         .redis_connection_info("redis://127.0.0.1/")
//!         .build()
//!         .unwrap();
//!
//!     // Serve!
//!     let _ = server.serve("127.0.0.1:8000");
//! }
//! ```
//!
//! Now you can build and run your binary with `cargo run`. Running this code
//! starts a HTTP server instance on port 8000. You can also override the port
//! by setting the `PORT` environment variable.
//!
//! See the
//! [`examples/`](https://github.com/spaceapi-community/spaceapi-server-rs/tree/master/examples)
//! directory for some other examples.
//!
//!
//! ## Sensors
//!
//! ### Registering Sensors
//!
//! This crate supports updating and retrieving dynamic sensor values (e.g.
//! temperature or people present). For this, first register a sensor with a
//! sensor template:
//!
//! ```rust
//! use spaceapi_server::SpaceapiServerBuilder;
//! use spaceapi_server::api::sensors::{PeopleNowPresentSensorTemplate, TemperatureSensorTemplate};
//!
//! # use spaceapi_server::api;
//! # let status = api::StatusBuilder::v14("aa")
//! #     .logo("https://example.com/logo.png")
//! #     .url("https://example.com/")
//! #     .location(api::Location {
//! #         address: Some("addr".into()),
//! #         lat: 47.0,
//! #         lon: 8.0,
//! #     })
//! #     .contact(api::Contact {
//! #         twitter: Some("@example".into()),
//! #         ..Default::default()
//! #     })
//! #     .build()
//! #     .expect("Creating status failed");
//! #
//! let server = SpaceapiServerBuilder::new(status)
//!     .redis_connection_info("redis://127.0.0.1/")
//!     .add_sensor(PeopleNowPresentSensorTemplate {
//!         location: Some("Hackerspace".into()),
//!         name: None,
//!         description: None,
//!         names: None,
//!     }, "people_now_present".into())
//!     .add_sensor(TemperatureSensorTemplate {
//!         unit: "°C".into(),
//!         location: "Room 1".into(),
//!         name: None,
//!         description: None,
//!     }, "temp_room1".into())
//!     .add_sensor(TemperatureSensorTemplate {
//!         unit: "°C".into(),
//!         location: "Room 2".into(),
//!         name: None,
//!         description: None,
//!     }, "temp_room2".into())
//!     .build()
//! .expect("Could not initialize server");
//! ```
//!
//! (You can find the full example at
//! [`examples/with_sensors.rs`](https://github.com/spaceapi-community/spaceapi-server-rs/blob/master/examples/with_sensors.rs).)
//!
//! This will register three sensors: One "people now present" sensor and two
//! "temperature" sensors.
//!
//! ### Updating Sensors via HTTP
//!
//! If you start the server like that, the JSON output will not yet contain any
//! sensor data. To update a sensor value, send a HTTP POST request to the
//! `/sensors/<sensor-id>/` endpoint with the `value` parameter:
//!
//! ```text
//! curl -v -X PUT -d value=42 http://127.0.0.1:8000/sensors/people_now_present/
//! curl -v -X PUT -d value=13.37 http://127.0.0.1:8000/sensors/temp_room1/
//! ```
//!
//! Now the server response will contain the following key:
//!
//! ```json
//! "sensors": {
//!   "people_now_present": [
//!     {
//!       "location": "Hackerspace",
//!       "value": 42
//!     }
//!   ],
//!   "temperature": [
//!     {
//!       "unit": "°C",
//!       "location": "Room 1",
//!       "value": 13.37
//!     }
//!   ]
//! },
//! ```
//!
//! ### Updating Sensors via Redis
//!
//! Alternatively you can modify the values in Redis directly. You can access
//! the database with the `redis-cli` tool:
//!
//! ```text
//! % redis-cli
//! 127.0.0.1:6379> SET people_now_present 1
//! OK
//! 127.0.0.1:6379> GET people_now_present
//! "1"
//! 127.0.0.1:6379> KEYS *
//! 1) "people_now_present"
//! 2) "temp_room1"
//! ```
//!
//! The keys need to match the IDs you used when registering the sensor.

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/spaceapi-server")]

pub use spaceapi as api;

pub use iron::error::HttpResult;
pub use iron::Listening;

mod errors;
pub mod modifiers;
mod sensors;
mod server;
mod types;

pub use crate::errors::SpaceapiServerError;
pub use crate::server::SpaceapiServer;
pub use crate::server::SpaceapiServerBuilder;

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
