use crate::{
    service::{cache_service::CacheService, crud_service::CrudService},
    APPLICATION_CONTEXT,
};
use cassie_common::error::Result;
use cassie_domain::{dto::sys_event_dto::EventConfigDTO, entity::event::EventConfig, request::EventQuery};
use rbatis::rbatis::Rbatis;

pub struct EventConfigService {}
impl EventConfigService {}
impl CrudService<EventConfig, EventConfigDTO, EventQuery> for EventConfigService {
    fn get_wrapper(arg: &EventQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }

    fn set_save_common_fields(&self, common: cassie_domain::entity::sys_entitys::CommonField, data: &mut EventConfig) {
        data.id = common.id;
    }
}
//加载全量事件配置到缓存中
pub async fn load_event() -> Result<Vec<EventConfigDTO>> {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    match cache_service.get_json::<Vec<EventConfigDTO>>("event_config").await {
        Ok(list) => Ok(list),
        Err(e) => {
            let service = APPLICATION_CONTEXT.get::<EventConfigService>();
            let query = vec!["0".to_string()];
            let list = service.fetch_list_by_column(EventConfig::status(), &query).await.unwrap();
            cache_service.set_json("event_config", &list).await;
            Ok(list)
        }
    }
}
