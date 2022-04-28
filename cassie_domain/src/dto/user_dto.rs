use serde::{Deserialize, Serialize};

use crate::entity::user_entity::{User, WechatUser};
#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct UserDTO {
  id: Option<i64>,
  account: Option<String>,
  pwd: Option<String>,
  real_name: Option<String>,
  birthday: Option<i64>,
  card_id: Option<String>,
  mark: Option<String>,
  group_id: Option<i64>,
  nickname: Option<String>,
  avatar: Option<String>,
  phone: Option<String>,
  add_time: Option<i32>,
  last_time: Option<i32>,
  status: Option<u8>,
  user_type: Option<String>,
  pay_count: Option<u32>,
  addres: Option<String>,
  login_type: Option<String>,
}

impl Into<User> for UserDTO {
  fn into(self) -> User {
    User {
      id: self.id().clone(),
      account: self.account().clone(),
      pwd: self.pwd().clone(),
      real_name: self.real_name().clone(),
      birthday: self.birthday().clone(),
      card_id: self.card_id().clone(),
      mark: self.mark().clone(),
      group_id: self.group_id().clone(),
      nickname: self.nickname().clone(),
      avatar: self.avatar().clone(),
      phone: self.phone().clone(),
      add_time: self.add_time().clone(),
      last_time: self.last_time().clone(),
      status: self.status().clone(),
      user_type: self.user_type().clone(),
      pay_count: self.pay_count().clone(),
      addres: self.addres().clone(),
      login_type: self.login_type().clone(),
    }
  }
}

impl From<User> for UserDTO {
  fn from(arg: User) -> Self {
    Self {
      id: arg.id,
      account: arg.account,
      pwd: arg.pwd,
      real_name: arg.real_name,
      birthday: arg.birthday,
      card_id: arg.card_id,
      mark: arg.mark,
      group_id: arg.group_id,
      nickname: arg.nickname,
      avatar: arg.avatar,
      phone: arg.phone,
      add_time: arg.add_time,
      last_time: arg.last_time,
      status: arg.status,
      user_type: arg.user_type,
      pay_count: arg.pay_count,
      addres: arg.addres,
      login_type: arg.login_type,
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct WechatUserDTO {
  id: Option<i64>,
  unionid: Option<String>,
  openid: Option<String>,
  routine_openid: Option<String>,
  nickname: Option<String>,
  headimgurl: Option<String>,
  sex: Option<u8>,
  city: Option<String>,
  language: Option<String>,
  province: Option<String>,
  country: Option<String>,
  remark: Option<i32>,
  groupid: Option<i32>,
  user_type: Option<String>,
  session_key: Option<String>,
}

impl Into<WechatUser> for WechatUserDTO {
  fn into(self) -> WechatUser {
    WechatUser {
      id: self.id().clone(),
      unionid: self.unionid().clone(),
      openid: self.openid().clone(),
      routine_openid: self.routine_openid().clone(),
      nickname: self.nickname().clone(),
      headimgurl: self.headimgurl().clone(),
      sex: self.sex().clone(),
      city: self.city().clone(),
      language: self.language().clone(),
      province: self.province().clone(),
      country: self.country().clone(),
      remark: self.remark().clone(),
      groupid: self.groupid().clone(),
      user_type: self.user_type().clone(),
      session_key: self.session_key().clone(),
    }
  }
}

impl From<WechatUser> for WechatUserDTO {
  fn from(arg: WechatUser) -> Self {
    Self {
      id: arg.id,
      unionid: arg.unionid,
      openid: arg.openid,
      routine_openid: arg.routine_openid,
      nickname: arg.nickname,
      headimgurl: arg.headimgurl,
      sex: arg.sex,
      city: arg.city,
      language: arg.language,
      province: arg.province,
      country: arg.country,
      remark: arg.remark,
      groupid: arg.groupid,
      user_type: arg.user_type,
      session_key: arg.session_key,
    }
  }
}
