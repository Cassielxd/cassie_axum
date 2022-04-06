use crate::entity::event::EventConfig;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventConfigDTO {
    pub id: Option<i64>,
    pub status: Option<String>,
    pub extend1: Option<String>,
    pub extend2: Option<String>,
    pub extend3: Option<String>,
    pub description: Option<String>,
    pub agency_code: Option<String>,
    pub path: Option<String>,
    pub resource_name: Option<String>,
    pub oprate_description: Option<String>,
    pub event_type: Option<String>,
    pub event_code: Option<String>,
    pub event_name: Option<String>,
    pub lock_user_id: Option<String>,
    pub event_script: Option<String>,
    pub need_persist: Option<String>,
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
