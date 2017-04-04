extern crate spaceapi_server;
extern crate env_logger;

use spaceapi_server::SpaceapiServer;
use spaceapi_server::api;
use spaceapi_server::api::sensors::PeopleNowPresentSensorTemplate;
use spaceapi_server::modifiers::{StatusModifier, StateFromPeopleNowPresent};


fn main() {
    env_logger::init().unwrap();

    // Create new minimal Status instance
    let status = api::StatusBuilder::new("coredump")
        .logo("https://www.coredump.ch/logo.png")
        .url("https://www.coredump.ch/")
        .location(api::Location {
            address: Some("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".into()),
            lat: 47.22936,
            lon: 8.82949,
        })
        .contact(api::Contact {
            irc: Some("irc://freenode.net/#coredump".into()),
            twitter: Some("@coredump_ch".into()),
            ..Default::default()
        })
        .add_issue_report_channel("email")
        .add_issue_report_channel("twitter")
        .build()
        .expect("Creating status failed");

    // Set up server
    let listen = "127.0.0.1:8000";
    let redis = "redis://127.0.0.1/";
    let modifiers: Vec<Box<StatusModifier + 'static>> = vec![Box::new(StateFromPeopleNowPresent)];
    let mut server = SpaceapiServer::new(listen, status, redis, modifiers)
                                    .expect("Could not initialize server");

    // Register sensors
    server.register_sensor(Box::new(PeopleNowPresentSensorTemplate {
        location: Some("Hackerspace".into()),
        name: None,
        description: None,
        names: None,
    }), "people_now_present".into());

    // Serve!
    server.serve().expect("Could not start the server");
}
