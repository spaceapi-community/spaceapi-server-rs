extern crate spaceapi_server;

use std::net::Ipv4Addr;
use std::net::TcpStream;
use std::io::ErrorKind;

use spaceapi_server::SpaceapiServer;
use spaceapi_server::api;
use spaceapi_server::api::optional::Optional;


/// Create a new status object containing test data.
fn get_status() -> api::Status {
    api::Status::new(
        "ourspace",
        "https://example.com/logo.png",
        "https://example.com/",
        api::Location {
            address: Optional::Value("Street 1, Zürich, Switzerland".into()),
            lat: 47.123,
            lon: 8.88,
        },
        api::Contact {
            irc: Optional::Absent,
            twitter: Optional::Absent,
            foursquare: Optional::Absent,
            email: Optional::Value("hi@example.com".into()),
        },
        vec![
            "email".into(),
            "twitter".into(),
        ],
    )
}


/// Create a new SpaceapiServer instance listening on the specified port.
fn get_server(ip: Ipv4Addr, port: u16, status: api::Status) -> SpaceapiServer {
    // Start and return a server instance
    SpaceapiServer::new((ip, port), status, "redis://127.0.0.1/", vec![]).unwrap()
}


#[test]
fn server_starts() {
    //! Test that the spaceapi server starts at all.

    // Ip / port for test server
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 3344;

    // Test data
    let status = get_status();

    // Connection to port should fail right now
    let connect_result = TcpStream::connect((ip, port));
    assert!(connect_result.is_err());
    assert_eq!(connect_result.unwrap_err().kind(), ErrorKind::ConnectionRefused);

    // Instantiate and start server
    let server = get_server(ip, port, status);
    let mut listening = server.serve().unwrap();

    // Connecting to server should work now
    let connect_result = TcpStream::connect((ip, port));
    assert!(connect_result.is_ok());

    // Close server
    listening.close().unwrap();
}
