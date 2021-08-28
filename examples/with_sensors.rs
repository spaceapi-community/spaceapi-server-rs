use spaceapi_server::api;
use spaceapi_server::api::sensors::{PeopleNowPresentSensorTemplate, TemperatureSensorTemplate};
use spaceapi_server::modifiers::StateFromPeopleNowPresent;
use spaceapi_server::SpaceapiServerBuilder;

fn main() {
    env_logger::init();

    // Create new minimal Status instance compatible with v0.13 and v14
    let status = api::StatusBuilder::mixed("coredump")
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
        .add_issue_report_channel(api::IssueReportChannel::Email)
        .add_issue_report_channel(api::IssueReportChannel::Twitter)
        .state(api::State::default())
        .build()
        .expect("Creating status failed");

    // Set up server
    let server = SpaceapiServerBuilder::new(status)
        .redis_connection_info("redis://127.0.0.1/")
        .add_status_modifier(StateFromPeopleNowPresent)
        .add_sensor(
            PeopleNowPresentSensorTemplate {
                location: Some("Hackerspace".into()),
                name: None,
                description: None,
                names: None,
            },
            "people_now_present".into(),
        )
        .add_sensor(
            TemperatureSensorTemplate {
                unit: "°C".into(),
                location: "Room 1".into(),
                name: None,
                description: None,
            },
            "temp_room1".into(),
        )
        .add_sensor(
            TemperatureSensorTemplate {
                unit: "°C".into(),
                location: "Room 2".into(),
                name: None,
                description: None,
            },
            "temp_room2".into(),
        )
        .build()
        .expect("Could not initialize server");

    // Serve!
    server
        .serve("127.0.0.1:8000")
        .expect("Could not start the server");
}
