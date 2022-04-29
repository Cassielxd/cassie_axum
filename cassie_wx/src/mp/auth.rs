use crate::sdk::constant::API_DOMAIN;
use crate::{
    json_decode,
    sdk::{client::Client, constant::WECHAT_OPEN_URI},
};
use cassie_common::error::Result;
use serde_json::Value;

use super::{WxMpAccessToken, WxMpUserInfo};

//用于公众号后端主动 发起获取用户的基础信息  access_token是服务号的
pub async fn get_mp_user_info(open_id: &str, access_token: &str) -> Result<Value> {
    let url = format!("{}/cgi-bin/user/info?access_token={}&openid={}&lang=Language.zh_CN", WECHAT_OPEN_URI, access_token, open_id);
    let api = Client::new();
    let res = api.get(&url).await?;
    let data = json_decode(&res)?;
    Ok(data)
}

//获取网页授权的access_token
pub async fn get_user_access_token(code: &str, appid: &str, secret: &str) -> Result<WxMpAccessToken> {
    let url = format!("{}/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code", API_DOMAIN, appid, secret, code);
    let api = Client::new();
    let res = api.get(&url).await?;
    let data = json_decode(&res)?;
    let res: WxMpAccessToken = serde_json::from_value(data).unwrap();
    Ok(res)
}

//根据用户 access_token ，openid获取用户信息 需scope为 snsapi_userinfo
pub async fn get_user_info(access_token: &str, openid: &str) -> Result<WxMpUserInfo> {
    let url = format!("{}/sns/userinfo?access_token={}&openid={}&lang=zh_CN", API_DOMAIN, access_token, openid);
    let api = Client::new();
    let res = api.get(&url).await?;
    let data = json_decode(&res)?;
    let res: WxMpUserInfo = serde_json::from_value(data).unwrap();
    Ok(res)
}
