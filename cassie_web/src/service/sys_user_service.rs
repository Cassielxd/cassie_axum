use crate::REQUEST_CONTEXT;
use crate::{dto::sys_user_dto::SysUserDTO, entity::sys_entitys::SysUser, request::SysUserQuery};
use rbatis::wrapper::Wrapper;

use super::{crud_service::CrudService, CONTEXT};
use crate::entity::sys_entitys::CommonField;
use crate::cici_casbin::CASBIN_CONTEXT;
use casbin::MgmtApi;

/**
 *struct:SysUserService
 *desc:用户基础服务
 *author:String
 *email:348040933@qq.com
 */
pub struct SysUserService {}
impl Default for SysUserService{
    fn default() -> Self {
        SysUserService{}
    }
}

impl SysUserService {
    pub async fn save_info(&self, arg: SysUserDTO) {
        let role_id = arg.role_id.clone();
        let mut entity: SysUser = arg.into();
        let tls = REQUEST_CONTEXT.clone();
        let (creator,agency_code) = if let Some(a) = tls.get() {
            (a.uid as i64,a.agency_code.clone())
        } else {
            (0,"".to_string())
        };
        entity.agency_code = Some(agency_code.clone());
        /*保存到数据库*/
       let uid = self.save(&mut entity).await;
        if let Some(rid) = role_id {
            let cached_enforcer = CASBIN_CONTEXT.enforcer.clone();
            let mut lock = cached_enforcer.write().await;
            lock.add_grouping_policy(vec![
                uid.unwrap().to_string(),
                rid.to_string(),
                agency_code.clone(),
            ]);
            drop(lock);
        }
    }
}

impl CrudService<SysUser, SysUserDTO, SysUserQuery> for SysUserService {
    fn get_wrapper(arg: &SysUserQuery) -> Wrapper {
        CONTEXT.rbatis.new_wrapper().eq(SysUser::del_flag(), 0)
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut SysUser) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
