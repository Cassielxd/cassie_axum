#[crud_table(table_name:user)]
#[derive(Clone, Debug)]
pub struct User {
  pub id: Option<i64>,
  pub account: Option<String>,
  pub pwd: Option<String>,
  pub real_name: Option<String>,
  pub birthday: Option<i64>,
  pub card_id: Option<String>,
  pub mark: Option<String>,
  pub group_id: Option<i64>,
  pub nickname: Option<String>,
  pub avatar: Option<String>,
  pub phone: Option<String>,
  pub add_time: Option<i32>,
  pub last_time: Option<i32>,
  pub status: Option<u8>,
  pub user_type: Option<String>,
  pub pay_count: Option<u32>,
  pub addres: Option<String>,
  pub login_type: Option<String>,
}

impl_field_name_method!(User {
  id,
  account,
  pwd,
  birthday,
  real_name,
  card_id,
  mark,
  group_id,
  nickname,
  avatar,
  phone,
  add_time,
  status,
  user_type,
  pay_count,
  addres,
  login_type,
});

#[crud_table(table_name:wechat_user)]
#[derive(Clone, Debug)]
pub struct WechatUser {
  pub id: Option<i64>,
  pub unionid: Option<String>,
  pub openid: Option<String>,
  pub routine_openid: Option<String>,
  pub nickname: Option<String>,
  pub headimgurl: Option<String>,
  pub sex: Option<u8>,
  pub city: Option<String>,
  pub language: Option<String>,
  pub province: Option<String>,
  pub country: Option<String>,
  pub remark: Option<i32>,
  pub groupid: Option<i32>,
  pub user_type: Option<String>,
  pub session_key: Option<String>,
}
impl_field_name_method!(WechatUser {
  id,
  unionid,
  openid,
  routine_openid,
  nickname,
  headimgurl,
  sex,
  city,
  language,
  province,
  country,
  remark,
  groupid,
  user_type
});
