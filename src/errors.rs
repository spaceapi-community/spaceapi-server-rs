//! Custom error types.

use std::borrow::Cow;
use std::io;

use quick_error::quick_error;
use r2d2::Error as R2d2Error;
use redis::RedisError;

quick_error! {
    /// A ``SpaceapiServerError`` wraps general problems that can occur in the SpaceAPI server.
    #[derive(Debug)]
    pub enum SpaceapiServerError {
        /// A problem with redis occurred.
        Redis(err: RedisError) {
            from()
            source(err)
        }
        /// A problem with the redis connection pool occurred.
        R2d2(err: R2d2Error) {
            from()
            source(err)
        }
        /// An I/O error occurred.
        IoError(err: io::Error) {
            from()
            source(err)
        }
        /// Another error happened.
        Message(err: Cow<'static, str>) {
            display("{}", err)
            from(s: &'static str) -> (s.into())
            from(s: String) -> (s.into())
        }
    }
}
