use cassie_domain::{
    dto::user_dto::{UserDTO, WechatUserDTO},
    entity::user_entity::{User, WechatUser},
    request::UserQuery,
};
use rbatis::rbatis::Rbatis;

use crate::{service::crud_service::CrudService, APPLICATION_CONTEXT};

pub struct UserService;

impl UserService {}

impl CrudService<User, UserDTO, UserQuery> for UserService {
    fn get_wrapper(arg: &UserQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }

    fn set_save_common_fields(
        &self,
        common: cassie_domain::entity::sys_entitys::CommonField,
        data: &mut User,
    ) {
    }
}

pub struct WechatUserService;

impl WechatUserService {}

impl CrudService<WechatUser, WechatUserDTO, UserQuery> for WechatUserService {
    fn get_wrapper(arg: &UserQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = rb.new_wrapper();
        rb.new_wrapper().do_if(arg.unionid().is_some(), |w| {
            w.eq(WechatUser::unionid(), &arg.unionid())
        })
    }

    fn set_save_common_fields(
        &self,
        common: cassie_domain::entity::sys_entitys::CommonField,
        data: &mut WechatUser,
    ) {
    }
}
//新增或更新用户
pub async fn save_or_update_user(user: WechatUserDTO) {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let wechat_user_service = APPLICATION_CONTEXT.get::<WechatUserService>();

    if user.unionid().is_some() && {
        let mut query = UserQuery::default();
        query.set_unionid(user.unionid().clone());
        let count = wechat_user_service
            .fetch_count_by_wrapper(&query)
            .await
            .unwrap();
        if count > 0 {
            true
        } else {
            false
        }
    } {
        //执行更新逻辑
    }
}
