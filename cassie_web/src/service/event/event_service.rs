use crate::{service::crud_service::CrudService, APPLICATION_CONTEXT};
use cassie_common::error::Result;
use cassie_domain::{
    dto::sys_event_dto::EventConfigDTO, entity::event::EventConfig, request::EventQuery,
};
use rbatis::rbatis::Rbatis;

pub struct EventConfigService {}
impl EventConfigService {
    pub async fn load_event(&self) -> Result<Vec<EventConfigDTO>> {
        let query = vec!["0".to_string()];
        let list = self
            .fetch_list_by_column(EventConfig::status(), &query)
            .await;
        list
    }
}
impl CrudService<EventConfig, EventConfigDTO, EventQuery> for EventConfigService {
    fn get_wrapper(arg: &EventQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }

    fn set_save_common_fields(
        &self,
        common: cassie_domain::entity::sys_entitys::CommonField,
        data: &mut EventConfig,
    ) {
        data.id = common.id;
    }
}
