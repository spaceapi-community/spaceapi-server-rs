/// Modifiers which can be injected by the application logic to change the state

use serde_json::Value;
use serde_json::map::Map;

use ::api;

/// `StatusModifier`s are used to modify the status
pub trait StatusModifier: Send + Sync {
    /// Called after all registered sensors are read
    fn modify(&self, status: &mut api::Status);
}

/// This modifier updates the opening state based on the
/// people now present sensor.
pub struct StateFromPeopleNowPresent;

impl StatusModifier for StateFromPeopleNowPresent {
    fn modify(&self, status: &mut api::Status) {
        // Update state depending on number of people present
        let people_now_present: Option<u64> = status.sensors.as_ref()
            .map(|sensors| sensors.people_now_present[0].value)
            .into();
        if let Some(count) = people_now_present {
            status.state.open = Some(count > 0);
            if count == 1 {
                status.state.message = Some(format!("{} person here right now", count));
            } else if count > 1 {
                status.state.message = Some(format!("{} people here right now", count));
            }
        }
    }
}

/// This modifier adds internal version information to the output.
pub struct LibraryVersions;

impl StatusModifier for LibraryVersions {
    fn modify(&self, status: &mut api::Status) {
        // Instantiate versions object
        let mut versions = Map::new();
        versions.insert("spaceapi-rs".into(), api::get_version().into());
        versions.insert("spaceapi-server-rs".into(), ::get_version().into());

        // Add to extensions
        status.extensions.insert("versions".into(), Value::Object(versions));
    }
}
