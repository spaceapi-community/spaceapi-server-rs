extern crate spaceapi_server;
extern crate env_logger;

use spaceapi_server::SpaceapiServer;
use spaceapi_server::api;
use spaceapi_server::api::Optional::{Value, Absent};
use spaceapi_server::api::sensors::PeopleNowPresentSensorTemplate;
use spaceapi_server::modifiers::{StatusModifier, StateFromPeopleNowPresent};


fn main() {
    env_logger::init().unwrap();

    // Create new minimal Status instance
    let status = api::Status::new(
        "coredump",
        "https://www.coredump.ch/logo.png",
        "https://www.coredump.ch/",
        api::Location {
            address: Value("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".into()),
            lat: 47.22936,
            lon: 8.82949,
        },
        api::Contact {
            irc: Value("irc://freenode.net/#coredump".into()),
            twitter: Value("@coredump_ch".into()),
            foursquare: Absent,
            email: Absent,
            ml: Absent,
            phone: Absent,
            jabber: Absent,
            issue_mail: Absent,
            identica: Absent,
            facebook: Absent,
            google: Absent,
            keymasters: Absent,
            sip: Absent,
        },
        vec![
            "email".into(),
            "twitter".into(),
        ],
    );

    // Set up server
    let listen = "127.0.0.1:8000";
    let redis = "redis://127.0.0.1/";
    let modifiers: Vec<Box<StatusModifier + 'static>> = vec![Box::new(StateFromPeopleNowPresent)];
    let mut server = SpaceapiServer::new(listen, status, redis, modifiers)
                                    .expect("Could not initialize server");

    // Register sensors
    server.register_sensor(Box::new(PeopleNowPresentSensorTemplate {
        location: Value("Hackerspace".into()),
        name: Absent,
        description: Absent,
        names: Absent,
    }), "people_now_present".into());

    // Serve!
    server.serve().expect("Could not start the server");
}
