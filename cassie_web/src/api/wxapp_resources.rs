use std::collections::HashMap;

use axum::{response::IntoResponse, Json};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_config::config::ApplicationConfig;
use cassie_domain::dto::user_dto::WechatUserDTO;
use cassie_wx::wxapp::auth::get_session_key;
use rbatis::Uuid;

use crate::{
    service::{api::user_service::save_or_update_user, cache_service::CacheService},
    APPLICATION_CONTEXT,
};

//小程序授权登录
pub async fn mp_auth(Json(sign): Json<HashMap<String, String>>) -> impl IntoResponse {
    let cache_service = APPLICATION_CONTEXT.get::<CacheService>();

    let mut session_key = if sign.contains_key("cache_key") {
        match cache_service
            .get_string(&format!(
                "cassie_api_code_{}",
                sign.get("cache_key").clone().unwrap()
            ))
            .await
        {
            Ok(d) => d,
            Err(_) => "".to_string(),
        }
    } else {
        "".to_string()
    };
    if !sign.contains_key("code") && session_key.is_empty() {
        return RespVO::<()>::from_error(&Error::from("授权失败,参数有误!")).resp_json();
    }

    let mut cache_key = "".to_string();

    let mut wechat_user = WechatUserDTO::default();
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //新登录用户
    if sign.contains_key("code") && !session_key.is_empty() {
        match get_session_key(
            config.wxapp().appid(),
            config.wxapp().secret(),
            &sign.get("code").clone().unwrap(),
        )
        .await
        {
            Ok(data) => {
                session_key = data.get("session_key").unwrap().to_string();
                let openid = data.get("openid").unwrap().to_string();
                let unionid = if data.get("unionid").is_some() {
                    data.get("unionid").unwrap().to_string()
                } else {
                    "".to_string()
                };
                wechat_user.set_routine_openid(Some(openid));
                wechat_user.set_unionid(Some(unionid));
                //生成新的缓存key
                cache_key = Uuid::new().to_string();
                cache_service
                    .set_string(
                        &format!("cassie_api_code_{}", cache_key.clone()),
                        &session_key,
                    )
                    .await;
            }
            Err(e) => {
                return RespVO::<()>::from_error(&Error::from(
                    "获取session_key失败，请检查您的配置！",
                ))
                .resp_json();
            }
        };
        //解密获取 用户信息

        wechat_user.set_session_key(Some(session_key));
        save_or_update_user(wechat_user);
    }

    RespVO::from(&"".to_string()).resp_json()
}

//小程序支付回调
pub async fn notify() {}
