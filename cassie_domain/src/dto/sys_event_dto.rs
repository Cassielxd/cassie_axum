use crate::entity::event::EventConfig;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct EventConfigDTO {
    id: Option<i64>,
    status: Option<String>,
    extend1: Option<String>,
    extend2: Option<String>,
    extend3: Option<String>,
    description: Option<String>,
    agency_code: Option<String>,
    path: Option<String>,
    resource_name: Option<String>,
    oprate_description: Option<String>,
    event_type: Option<String>,
    event_code: Option<String>,
    event_name: Option<String>,
    lock_user_id: Option<String>,
    event_script: Option<String>,
    need_persist: Option<String>,
}

impl From<EventConfig> for EventConfigDTO {
    fn from(arg: EventConfig) -> Self {
        Self {
            id: arg.id,
            status: arg.status,
            extend1: arg.extend1,
            extend2: arg.extend2,
            extend3: arg.extend3,
            description: arg.description,
            agency_code: arg.agency_code,
            path: arg.path,
            resource_name: arg.resource_name,
            oprate_description: arg.oprate_description,
            event_type: arg.event_type,
            event_code: arg.event_code,
            event_name: arg.event_name,
            lock_user_id: arg.lock_user_id,
            event_script: arg.event_script,
            need_persist: arg.need_persist,
        }
    }
}

impl Into<EventConfig> for EventConfigDTO {
    fn into(self) -> EventConfig {
        EventConfig {
            id: self.id,
            status: self.status,
            extend1: self.extend1,
            extend2: self.extend2,
            extend3: self.extend3,
            description: self.description,
            agency_code: self.agency_code,
            path: self.path,
            resource_name: self.resource_name,
            oprate_description: self.oprate_description,
            event_type: self.event_type,
            event_code: self.event_code,
            event_name: self.event_name,
            lock_user_id: self.lock_user_id,
            event_script: self.event_script,
            need_persist: self.need_persist,
        }
    }
}
