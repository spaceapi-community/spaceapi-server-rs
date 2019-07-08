//! Custom error types.

use redis::RedisError;
use r2d2::InitializationError;
use std::io;
use std::borrow::Cow;


/// A ``SpaceapiServerError`` wraps general problems that can occur in the SpaceAPI server.
quick_error! {
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
