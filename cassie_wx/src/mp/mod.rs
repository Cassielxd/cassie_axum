use serde::{Deserialize, Serialize};

pub mod auth;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxMpUserInfo {
    pub openid: Option<String>,
    pub nickname: Option<String>,
    pub sex: Option<u8>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub headimgurl: Option<String>,
    pub unionid: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxMpAccessToken {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub openid: Option<String>,
}
