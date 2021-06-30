use spaceapi_server::api;
use spaceapi_server::SpaceapiServerBuilder;

fn main() {
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
        .build()
        .unwrap();

    // Serve!
    let _ = server.serve("127.0.0.1:8000");
}
