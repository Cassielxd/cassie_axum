use crate::{dto::sys_user_dto::SysUserDTO, entity::sys_entitys::SysUser, request::SysUserQuery};
use crate::{CONTEXT, RB, REQUEST_CONTEXT};
use cassie_common::utils::password_encoder::PasswordEncoder;
use rbatis::wrapper::Wrapper;

use super::crud_service::CrudService;
use super::sys_role_user_service::SysRoleUserService;
use crate::cici_casbin::CASBIN_CONTEXT;
use crate::entity::sys_entitys::{CommonField, SysRoleUser};
use casbin::MgmtApi;

/**
 *struct:SysUserService
 *desc:用户基础服务
 *author:String
 *email:348040933@qq.com
 */
pub struct SysUserService {
    pub sys_role_user_service :SysRoleUserService
}
impl Default for SysUserService {
    fn default() -> Self {
        SysUserService {sys_role_user_service:SysRoleUserService{}}
    }
}
impl SysUserService {

    pub async fn delete_user(&self, id: String) {
       let user_info = self.get(id.clone()).await.unwrap();
       self.del(&id).await;
       self.sys_role_user_service.del_by_column(SysRoleUser::user_id(),id.clone().as_str()).await;
       let cached_enforcer = CASBIN_CONTEXT.enforcer.clone();
            let mut lock = cached_enforcer.write().await;
            lock.remove_grouping_policy(vec![id]).await;
            drop(lock);
    }

    pub async fn save_info(&self, arg: SysUserDTO) {
        let password = PasswordEncoder::encode(&arg.password.clone().unwrap().as_str());
        let role_id = arg.role_id.clone();
        let mut entity: SysUser = arg.into();
        let tls = REQUEST_CONTEXT.clone();
        let (creator, agency_code) = if let Some(a) = tls.get() {
            (a.uid as i64, a.agency_code.clone())
        } else {
            (0, "".to_string())
        };
        entity.password = Some(password);
        entity.agency_code = Some(agency_code.clone());
        entity.super_admin = Some(0);
        entity.del_flag = Some(0);
        /*保存到数据库*/
        let uid = if let Some(id) = entity.id {
            self.update_by_id(id.to_string(), &entity).await;
            id
        } else {
            let id = self.save(&mut entity).await;
            id.unwrap()
        };
       let mut data = SysRoleUser{ id: None, role_id, user_id: Some(uid), creator: None, create_date: None };
        self.sys_role_user_service.save(&mut data).await;
        if let Some(rid) = role_id {
            let cached_enforcer = CASBIN_CONTEXT.enforcer.clone();
            let mut lock = cached_enforcer.write().await;
            lock.add_grouping_policy(vec![uid.to_string(), rid.to_string(), agency_code.clone()]).await;
            drop(lock);
        }
    }
}

impl CrudService<SysUser, SysUserDTO, SysUserQuery> for SysUserService {
    fn get_wrapper(arg: &SysUserQuery) -> Wrapper {
        RB.new_wrapper().eq(SysUser::del_flag(), 0)
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut SysUser) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
