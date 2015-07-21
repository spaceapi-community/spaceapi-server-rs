//! The main entry point for the Space API server.
//!
//! Running this code starts a HTTP server instance. The default port is 3000, but you can set your
//! own favorite port by exporting the `PORT` environment variable.

extern crate rustc_serialize;
extern crate iron;
extern crate spaceapi;

pub mod datastore;
pub mod redis_store;

use std::net::Ipv4Addr;
use std::sync::Mutex;
use std::sync::Arc;

use rustc_serialize::json::ToJson;
use iron::{Request, Response, IronResult, Iron, Set};
use iron::{status, headers, middleware};
use iron::modifiers::Header;

pub use datastore::DataStore;
use spaceapi::Optional::{Value, Absent};


fn build_response_json(status: &spaceapi::Status, people_present: Option<u32>, raspi_temperature: Option<f32>) -> String {
    let people_present_sensor = match people_present {
        Some(count) => Value(vec![
            spaceapi::PeopleNowPresentSensor {
                value: count,
                location: Value("Hackerspace".to_string()),
                name: Absent,
                description: Absent,
                names: Absent,
            }
        ]),
        None => Absent,
    };

    let temperature_sensor = match raspi_temperature {
        Some(degrees) => Value(vec![
            spaceapi::TemperatureSensor {
                value: degrees,
                unit: "°C".to_string(),
                location: "Hackerspace".to_string(),
                name: Value("Raspberry CPU".to_string()),
                description: Absent,
            }
        ]),
        None => Absent,
    };

    // Create a mutable copy of the status struct and add sensor data.
    let mut status_copy = (*status).clone();
    status_copy.sensors = Value(spaceapi::Sensors {
        people_now_present: people_present_sensor,
        temperature: temperature_sensor,
    });

    // Serialize to JSON string
    status_copy.to_json().to_string()
}


/// A Space API server instance.
///
/// You can create a new instance using the ``new`` constructor method by
/// passing it the host, the port, the ``Status`` object and a datastore.
///
/// The ``SpaceapiServer`` includes a web server through
/// [Hyper](http://hyper.rs/hyper/hyper/server/index.html). Simply call the ``serve`` method.
pub struct SpaceapiServer {
    host: Ipv4Addr,
    port: u16,
    status: spaceapi::Status,
    datastore: Arc<Mutex<Box<DataStore>>>,
}

impl SpaceapiServer {

    pub fn new(host: Ipv4Addr, port: u16, status: spaceapi::Status, datastore: Arc<Mutex<Box<DataStore>>>) -> SpaceapiServer{
        SpaceapiServer {
            host: host,
            port: port,
            status: status,
            datastore: datastore,
        }
    }

    pub fn serve(self) {
        let host = self.host;
        let port = self.port;
        println!("Starting HTTP server on {}:{}...", host, port);
        Iron::new(self).http((host, port)).unwrap();
    }

}

impl middleware::Handler for SpaceapiServer {

    fn handle(&self, _: &mut Request) -> IronResult<Response> {

        // Fetch data from datastore
        let datastore_clone = self.datastore.clone();
        let datastore_lock = datastore_clone.lock().unwrap();
        let people_present: Option<u32> = match datastore_lock.retrieve("people_present") {
            Ok(v) => match v.parse::<u32>() {
                Ok(i) => Some(i),
                Err(_) => None,
            },
            Err(_) => None,
        };
        let raspi_temperature: Option<f32> = match datastore_lock.retrieve("raspi_temperature") {
            Ok(v) => match v.parse::<f32>() {
                Ok(i) => Some(i),
                Err(_) => None,
            },
            Err(_) => None,
        };

        // Get response body
        let body = build_response_json(&self.status, people_present, raspi_temperature);

        // Create response
        let mut response = Response::with((status::Ok, body));

        // Set headers
        response.set_mut(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())));
        response.set_mut(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])));
        response.set_mut(Header(headers::AccessControlAllowOrigin::Any));

        Ok(response)
    }
}


#[cfg(test)]
mod test {
    use super::build_response_json;

    #[test]
    /// Verify that the response JSON looks OK.
    fn verify_response_json() {
        let people_present = Some(23);
        let temperature = Some(42.5);
        let json = build_response_json(people_present, temperature);
        assert_eq!(json, "{\
            \"api\":\"0.13\",\
            \"cache\":{\"schedule\":\"m.02\"},\
            \"contact\":{\
                \"email\":\"danilo@coredump.ch\",\
                \"foursquare\":\"525c20e5498e875d8231b1e5\",\
                \"irc\":\"irc://freenode.net/#coredump\",\
                \"twitter\":\"@coredump_ch\"\
            },\
            \"feeds\":{\
                \"blog\":{\"type\":\"rss\",\"url\":\"https://www.coredump.ch/feed/\"}\
            },\
            \"issue_report_channels\":[\"email\",\"twitter\"],\
            \"location\":{\
                \"address\":\"Spinnereistrasse 2, 8640 Rapperswil, Switzerland\",\
                \"lat\":47.22936,\"lon\":8.82949\
            },\
            \"logo\":\"https://www.coredump.ch/logo.png\",\
            \"projects\":[\
                \"https://www.coredump.ch/projekte/\",\
                \"https://discourse.coredump.ch/c/projects\",\
                \"https://github.com/coredump-ch/\"\
            ],\
            \"sensors\":{\
                \"people_now_present\":[{\
                    \"location\":\"Hackerspace\",\"value\":23\
                }],\
                \"temperature\":[{\
                    \"location\":\"Hackerspace\",\"name\":\"Raspberry CPU\",\
                    \"unit\":\"°C\",\"value\":42.5\
                }]\
            },\
            \"space\":\"coredump\",\
            \"spacefed\":{\"spacenet\":false,\"spacephone\":false,\"spacesaml\":false},\
            \"state\":{\"message\":\"Open every Monday from 20:00\",\"open\":false},\
            \"url\":\"https://www.coredump.ch/\"\
        }");
    }
}
