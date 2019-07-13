//! Custom error types.

use r2d2::InitializationError;
use redis::RedisError;
use std::borrow::Cow;
use std::io;

quick_error! {
    /// A ``SpaceapiServerError`` wraps general problems that can occur in the SpaceAPI server.
    #[derive(Debug)]
    pub enum SpaceapiServerError {
        Redis(err: RedisError) {
            from()
            cause(err)
        }
        R2d2(err: InitializationError) {
            from()
            cause(err)
        }
        IoError(err: io::Error) {
            from()
            cause(err)
        }
        Message(err: Cow<'static, str>) {
            description(&**err)
            from(s: &'static str) -> (s.into())
            from(s: String) -> (s.into())
        }
    }
}
