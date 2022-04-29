use rbatis::rbatis::Rbatis;

use crate::APPLICATION_CONTEXT;
use cassie_domain::entity::sys_entitys::CommonField;
use cassie_domain::{dto::sys_group_dto::SysGroupDTO, entity::sys_group_entity::SysGroup, request::SysGroupQuery};

use super::crud_service::CrudService;

pub struct SysGroupService {}
impl Default for SysGroupService {
    fn default() -> Self {
        SysGroupService {}
    }
}
impl CrudService<SysGroup, SysGroupDTO, SysGroupQuery> for SysGroupService {
    fn get_wrapper(arg: &SysGroupQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysGroup) {}
}
