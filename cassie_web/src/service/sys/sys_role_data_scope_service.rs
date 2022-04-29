use super::crud_service::CrudService;
use crate::middleware::get_local;
use crate::APPLICATION_CONTEXT;
use cassie_domain::entity::sys_entitys::CommonField;
use cassie_domain::{dto::sys_role_dto::SysRoleDataScopeDTO, entity::sys_entitys::SysRoleDataScope, request::SysRoleQuery};
use rbatis::plugin::snowflake::new_snowflake_id;
use rbatis::rbatis::Rbatis;
use rbatis::DateTimeNative;

/**
*struct:SysRoleDataScopeService
*desc:角色数据权限
*author:String
*email:348040933@qq.com
*/
pub struct SysRoleDataScopeService {}
impl Default for SysRoleDataScopeService {
  fn default() -> Self {
    SysRoleDataScopeService {}
  }
}
impl SysRoleDataScopeService {
  pub async fn save_or_update(&self, role_id: i64, dept_id_list: Option<Vec<i64>>) {
    //先删除角色数据权限关系
    self.delete_by_role_id(role_id).await;
    let request_model = get_local().unwrap();
    //保存角色数据权限关系
    if let Some(list) = dept_id_list {
      let mut vec = Vec::new();
      for x in list {
        vec.push(SysRoleDataScope {
          id: Some(new_snowflake_id()),
          role_id: Some(role_id),
          dept_id: Some(x),
          creator: Option::from(request_model.uid().clone()),
          create_date: Some(DateTimeNative::now()),
        });
      }
      self.save_batch(&vec).await;
    }
  }
  pub async fn delete_by_role_id(&self, role_id: i64) {
    self.del_by_column(SysRoleDataScope::role_id(), &role_id.to_string()).await;
  }
}
impl CrudService<SysRoleDataScope, SysRoleDataScopeDTO, SysRoleQuery> for SysRoleDataScopeService {
  fn get_wrapper(arg: &SysRoleQuery) -> rbatis::wrapper::Wrapper {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    rb.new_wrapper()
  }
  fn set_save_common_fields(&self, common: CommonField, data: &mut SysRoleDataScope) {
    data.id = common.id;
    data.creator = common.creator;
    data.create_date = common.create_date;
  }
}
