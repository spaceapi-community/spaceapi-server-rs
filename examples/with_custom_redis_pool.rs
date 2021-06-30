extern crate env_logger;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate spaceapi_server;

use spaceapi_server::api;
use spaceapi_server::modifiers::StatusModifier;
use spaceapi_server::SpaceapiServerBuilder;

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::{Commands, RedisResult};

type RedisPool = Pool<RedisConnectionManager>;

struct OpenStatusFromRedisModifier {
    pool: RedisPool,
}

impl OpenStatusFromRedisModifier {
    fn new(pool: RedisPool) -> OpenStatusFromRedisModifier {
        OpenStatusFromRedisModifier { pool }
    }
}

impl StatusModifier for OpenStatusFromRedisModifier {
    fn modify(&self, status: &mut api::Status) {
        let mut conn = self.pool.get().unwrap();
        let redis_state: RedisResult<String> = conn.get("state_open");
        if let Some(state) = &mut status.state {
            state.open = match redis_state {
                Ok(v) => Some(v == "open"),
                Err(_) => None,
            };
            state.lastchange = conn.get("state_lastchange").ok();
            state.trigger_person = conn.get("state_triggerperson").ok();
        }
    }
}

fn main() {
    env_logger::init();

    let status = api::StatusBuilder::mixed("Mittelab")
        .logo("https://www.mittelab.org/images/logo.svg")
        .url("https://www.mittelab.org")
        .location(api::Location {
            address: Some("Piazza Libertà 5/B, 34132 Trieste (TS), Italy".into()),
            lat: 45.656_652_6,
            lon: 13.773_387_2,
        })
        .contact(api::Contact {
            email: Some("info@mittelab.org".into()),
            irc: Some("irc://irc.freenode.net/#mittelab".into()),
            twitter: Some("@mittelab".into()),
            facebook: Some("https://facebook.com/mittelab".into()),
            phone: Some("+390409776431".into()),
            issue_mail: Some("sysadmin@mittelab.org".into()),
            ..Default::default()
        })
        .add_issue_report_channel(api::IssueReportChannel::Email)
        .add_project("https://git.mittelab.org")
        .add_project("https://github.com/mittelab")
        .add_project("https://wiki.mittelab.org/progetti/")
        .state(api::State::default())
        .build()
        .expect("Creating status failed");

    let manager = r2d2_redis::RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    // Set up server
    let server = SpaceapiServerBuilder::new(status)
        // .add_redis_pool(pool) waiting for
        .redis_pool(pool.clone())
        .add_status_modifier(OpenStatusFromRedisModifier::new(pool))
        .build()
        .expect("Could not initialize server");

    // Serve!
    server
        .serve("127.0.0.1:8000")
        .expect("Could not start the server");
}
