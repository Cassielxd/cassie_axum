use cassie_domain::{
  dto::user_dto::{UserDTO, WechatUserDTO},
  entity::user_entity::{User, WechatUser},
  request::UserQuery,
};
use rbatis::rbatis::Rbatis;

use crate::{service::crud_service::CrudService, APPLICATION_CONTEXT};

pub struct UserService;

impl CrudService<User, UserDTO, UserQuery> for UserService {
  fn get_wrapper(arg: &UserQuery) -> rbatis::wrapper::Wrapper {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    rb.new_wrapper()
  }
  fn set_save_common_fields(&self, common: cassie_domain::entity::sys_entitys::CommonField, data: &mut User) {}
}

pub struct WechatUserService;

impl WechatUserService {}

impl CrudService<WechatUser, WechatUserDTO, UserQuery> for WechatUserService {
  fn get_wrapper(arg: &UserQuery) -> rbatis::wrapper::Wrapper {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    let wrapper = rb.new_wrapper();
    rb.new_wrapper().do_if(arg.unionid().is_some(), |w| w.eq(WechatUser::unionid(), &arg.unionid()))
  }

  fn set_save_common_fields(&self, common: cassie_domain::entity::sys_entitys::CommonField, data: &mut WechatUser) {}
}
