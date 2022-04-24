use axum::{response::IntoResponse, Json};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_config::config::ApplicationConfig;
use cassie_domain::vo::wx::WxSignInVo;
use cassie_wx::wxapp::auth::get_session_key;

use crate::{service::cache_service::CacheService, APPLICATION_CONTEXT};

//小程序授权登录
pub async fn mp_auth(Json(sign): Json<WxSignInVo>) -> impl IntoResponse {
    if sign.code().is_none() {
        return RespVO::<()>::from_error(&Error::from("code不能为空!")).resp_json();
    }
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    let session_key = cache_service
        .get_string(&format!(
            "cassie_api_code_{}",
            sign.cache_key().clone().unwrap()
        ))
        .await;
    if sign.code().is_none() && sign.cache_key().is_none() {
        return RespVO::<()>::from_error(&Error::from("授权失败,参数有误!")).resp_json();
    }
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    if sign.code().is_none() && !sign.cache_key().is_none() {
        match get_session_key(
            config.wxapp().appid(),
            config.wxapp().secret(),
            &sign.code().clone().unwrap(),
        )
        .await
        {
            Ok(data) => {
                let session_key = data.get("session_key").unwrap();
                let openid = data.get("openid").unwrap();
                let unionid = if data.get("unionid").is_some() {
                    data.get("unionid").unwrap().to_string()
                } else {
                    "".to_string()
                };
                
            }
            Err(e) => {
                return RespVO::<()>::from_error(&Error::from(
                    "获取session_key失败，请检查您的配置！",
                ))
                .resp_json();
            }
        };
    }

    RespVO::from(&"".to_string()).resp_json()
}

//小程序支付回调
pub async fn notify() {}
