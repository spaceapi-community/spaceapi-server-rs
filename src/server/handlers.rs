//! Handlers for the server.

use std::sync::Arc;

use iron::{status, headers, middleware};
use iron::IronResult;
use iron::prelude::Set;
use iron::request::Request;
use iron::response::Response;
use iron::modifiers::Header;

use super::SpaceapiServer;


pub struct ReadHandler {
    server: Arc<SpaceapiServer>,
}

impl ReadHandler {
    pub fn new(server: Arc<SpaceapiServer>) -> ReadHandler {
        ReadHandler { server: server }
    }
}

impl middleware::Handler for ReadHandler {

    /// Return the current status JSON.
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        println!("{} /{} from {}", req.method, req.url.path[0], req.remote_addr);

        // Get response body
        let server = self.server.clone();
        let body = server.build_response_json().to_string();

        // Create response
        let response = Response::with((status::Ok, body))
            // Set headers
            .set(Header(headers::ContentType("application/json; charset=utf-8".parse().unwrap())))
            .set(Header(headers::CacheControl(vec![headers::CacheDirective::NoCache])))
            .set(Header(headers::AccessControlAllowOrigin::Any));

        Ok(response)
    }

}
