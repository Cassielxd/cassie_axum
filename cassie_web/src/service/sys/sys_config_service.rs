use cassie_domain::entity::sys_config_entity::SysConfig;
use cassie_domain::request::SysConfigQuery;
use rbatis::rbatis::Rbatis;

use crate::APPLICATION_CONTEXT;
use cassie_domain::dto::sys_config_dto::SysConfigDTO;
use cassie_domain::entity::sys_entitys::CommonField;

use super::crud_service::CrudService;

pub struct SysConfigService {}
impl Default for SysConfigService {
    fn default() -> Self {
        SysConfigService {}
    }
}
impl CrudService<SysConfig, SysConfigDTO, SysConfigQuery> for SysConfigService {
    fn get_wrapper(arg: &SysConfigQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysConfig) {}
}
