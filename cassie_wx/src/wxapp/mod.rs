use crate::sdk::aes_crypt::AesCrypt;
use cassie_common::error::Result;
use serde::{Deserialize, Serialize};
pub mod auth;

pub fn resolve_data(session_key: String, iv: String, encrypted_data: String) -> Result<WxUserInfo> {
  let key = base64::decode(session_key).unwrap();
  let iv = base64::decode(iv).unwrap();
  let aes = AesCrypt::new(key, iv);
  let data = aes.decrypt(encrypted_data);
  let info: WxUserInfo = serde_json::from_str(&data).unwrap();
  Ok(info)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxUserInfo {
  pub openid: Option<String>,
  pub nickName: Option<String>,
  pub gender: Option<u8>,
  pub language: Option<String>,
  pub city: Option<String>,
  pub province: Option<String>,
  pub country: Option<String>,
  pub avatarUrl: Option<String>,
  pub purePhoneNumber: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxappSessionKey {
  pub session_key: String,
  pub expires_in: i64,
  pub openid: String,
  pub unionid: Option<String>,
}
