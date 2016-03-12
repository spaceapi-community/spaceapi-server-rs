//! Custom error types.
//!
//! Unfortunately we can only define one error type using the ``error_type!`` macro in here, see
//! https://github.com/DanielKeep/rust-error-type/issues/2.

use redis::RedisError;
use std::io;
use std::borrow::Cow;


/// A ``SpaceapiServerError`` wraps general problems that can occur in the SpaceAPI server.
error_type! {
    #[derive(Debug)]
    pub enum SpaceapiServerError {
        Redis(RedisError) {
            cause;
        },
        IoError(io::Error) {
            cause;
        },
        Message(Cow<'static, str>) {
            desc (e) &**e;
            from (s: &'static str) s.into();
            from (s: String) s.into();
        },
    }
}
