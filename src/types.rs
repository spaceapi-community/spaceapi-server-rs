//! Type definitions.

use r2d2;
use r2d2_redis::RedisConnectionManager;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;
