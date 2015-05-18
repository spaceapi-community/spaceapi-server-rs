//! Some useful utils used in the SpaceAPI implementation.

use std::env;

/// Read the port number from the PORT env variable.
///
/// If the variable is not set or cannot be parsed to u16,
/// use 3000 as default port.
pub fn get_port() -> u16 {
    match env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(val) => val,
            Err(_) => 3000
        },
        Err(_) => 3000
    }
}
