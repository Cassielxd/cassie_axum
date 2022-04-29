use rbatis::rbatis::Rbatis;

use crate::APPLICATION_CONTEXT;
use cassie_domain::entity::sys_entitys::CommonField;
use cassie_domain::{dto::sys_config_tab_dto::SysConfigTabDTO, entity::sys_config_tab_entity::SysConfigTab, request::SysConfigTabQuery};

use super::crud_service::CrudService;

pub struct SysConfigTabService {}
impl Default for SysConfigTabService {
    fn default() -> Self {
        SysConfigTabService {}
    }
}
impl CrudService<SysConfigTab, SysConfigTabDTO, SysConfigTabQuery> for SysConfigTabService {
    fn get_wrapper(arg: &SysConfigTabQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysConfigTab) {}
}
