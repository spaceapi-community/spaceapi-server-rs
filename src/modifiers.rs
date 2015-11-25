/// Modifiers which can be injected by the application logic to change the state

use std::any::Any;
use ::api;
use ::api::optional::Optional;

/// `StatusModifier`s are used to modify the status
pub trait StatusModifier: Send + Sync {
    /// Called after all registered sensors are read
    fn modify(&self, status: &mut api::Status);
}

pub struct StateFromPeopleNowPresent;

impl StatusModifier for StateFromPeopleNowPresent {
    fn modify(&self, status: &mut api::Status) {
        // Update state depending on number of people present
        let people_now_present: Option<u64> = status.sensors.as_ref()
            .and_then(|sensors| sensors.people_now_present.as_ref())
            .map(|people_now_present| people_now_present[0].value)
            .into();
        if let Some(count) = people_now_present {
            status.state.open = Some(count > 0);
            if count == 1 {
                status.state.message = Optional::Value(format!("{} person here right now", count));
            } else if count > 1 {
                status.state.message = Optional::Value(format!("{} people here right now", count));
            }
        }
    }
}

