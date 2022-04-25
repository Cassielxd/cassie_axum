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
    pub openId: String,
    pub nickName: String,
    pub gender: u8,
    pub language: String,
    pub city: String,
    pub province: String,
    pub country: String,
    pub avatarUrl: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxappSessionKey {
    pub session_key: String,
    pub expires_in: i64,
    pub openid: String,
    pub unionid: Option<String>,
}
