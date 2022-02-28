use crate::CONTEXT;
use crate::entity::sys_entitys::CommonField;
use crate::{
    dto::sys_role_dto::SysRoleUserDTO, entity::sys_entitys::SysRoleUser, request::SysRoleQuery,
};

use super::{crud_service::CrudService};

/**
*struct:SysRoleUserService
*desc:角色用户关系
*author:String
*email:348040933@qq.com
*/
pub struct SysRoleUserService {}

impl Default for SysRoleUserService {
    fn default() -> Self {
        SysRoleUserService{}
    }
}
impl SysRoleUserService {}
impl CrudService<SysRoleUser, SysRoleUserDTO, SysRoleQuery> for SysRoleUserService {
    fn get_wrapper(arg: &SysRoleQuery) -> rbatis::wrapper::Wrapper {
        CONTEXT
            .rbatis
            .new_wrapper()
            .do_if(arg.user_id.is_some(), |w| {
                w.like(SysRoleUser::user_id(), &arg.user_id)
            })
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysRoleUser) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
