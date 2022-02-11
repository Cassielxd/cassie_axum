use super::{
    CONTEXT, crud_service::CrudService,
    sys_role_data_scope_service::SysRoleDataScopeService, sys_role_menu_service::SysRoleMenuService, sys_role_user_service::SysRoleUserService,
};
use crate::entity::sys_entitys::CommonField;
use cassie_common::error::Result;
use cassie_common::utils::string::IsEmpty;
use crate::{
    dto::sys_role_dto::SysRoleDTO, entity::sys_entitys::SysRole, request::SysRoleQuery,
};

/**
*struct:SysRoleService
*desc:角色基础服务
*author:String
*email:348040933@qq.com
*/
pub struct SysRoleService {
    pub sys_role_user_service: SysRoleUserService,
    pub sys_role_menu_service: SysRoleMenuService,
    pub sys_role_data_scope_service: SysRoleDataScopeService,
}
impl  Default for SysRoleService {
    fn default() -> Self {
        SysRoleService{
            sys_role_user_service: Default::default(),
            sys_role_menu_service: Default::default(),
            sys_role_data_scope_service: Default::default()
        }
    }
}
impl SysRoleService {
    pub async fn role_info(&self, uid_id: i64) -> Result<Vec<i64>> {
        let params = SysRoleQuery {
            id: None,
            name: None,
            dept_id: None,
            role_id: None,
            user_id: Some(uid_id),
            menu_id: None,
            page_no: None,
            page_size: None,
        };
        let user_role = self.sys_role_user_service.list(&params).await;
        if let Ok(list) = user_role {
            Ok(list
                .iter()
                .map(|f| f.role_id.clone().unwrap_or_default())
                .collect::<Vec<i64>>())
        } else {
            Ok(Vec::<i64>::new())
        }
    }
    pub async fn save_role(&self, sys_role: SysRoleDTO) {
        let dept_id_list = sys_role.dept_id_list.clone();
        let menu_id_list = sys_role.menu_id_list.clone();
        let mut entity = sys_role.into();
        //保存角色
        let role_id = self.save(&mut entity).await;
        if let Ok(id) = role_id {
            //保存角色菜单关系
            self.sys_role_menu_service
                .save_or_update(id, menu_id_list.clone())
                .await;
            //保存角色数据权限关系
            self.sys_role_data_scope_service
                .save_or_update(id, dept_id_list)
                .await;
        }
    }
}

impl CrudService<SysRole, SysRoleDTO, SysRoleQuery> for SysRoleService {
    fn get_wrapper(arg: &SysRoleQuery) -> rbatis::wrapper::Wrapper {
        CONTEXT
            .rbatis
            .new_wrapper()
            .do_if(!arg.name.is_empty(), |w| w.like(SysRole::name(), &arg.name))
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysRole) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
