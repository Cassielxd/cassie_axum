use std::collections::HashMap;

use axum::{response::IntoResponse, routing::post, Json, Router};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_config::config::ApplicationConfig;
use cassie_domain::{
    dto::user_dto::WechatUserDTO,
    vo::{jwt::JWTToken, sign_in::ApiSignInVO},
};
use cassie_wx::wxapp::{auth::get_session_key, resolve_data};
use rbatis::Uuid;

use crate::{
    service::{
        api::user_service::UserService, cache_service::CacheService, crud_service::CrudService,
        wechat::wxapp_service::save_or_update_user,
    },
    APPLICATION_CONTEXT,
};

//小程序授权登录
pub async fn mp_auth(Json(sign): Json<HashMap<String, String>>) -> impl IntoResponse {
    let mut cache_service = APPLICATION_CONTEXT.get::<CacheService>();
    //获取 session_key 如果已经授权了  直接拿到session_key
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
    //如果授权code 和session_key 都不存在 则参数异常
    if !sign.contains_key("code") && session_key.is_empty() {
        return RespVO::<()>::from_error(&Error::from("授权失败,参数有误!")).resp_json();
    }

    let mut cache_key = "".to_string();
    let mut unionid = "".to_string();
    let mut openid = "".to_string();
    let mut wechat_user = WechatUserDTO::default();
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //新登录用户
    //如果code存在 session_key不存在 则需要根据code拿到session_key
    if sign.contains_key("code") && session_key.is_empty() {
        match get_session_key(
            config.wxapp().appid(),
            config.wxapp().secret(),
            &sign.get("code").clone().unwrap(),
        )
        .await
        {
            Ok(data) => {
                session_key = data.get("session_key").unwrap().to_string();
                openid = data.get("openid").unwrap().to_string();
                unionid = if data.get("unionid").is_some() {
                    data.get("unionid").unwrap().to_string()
                } else {
                    "".to_string()
                };
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
    }

    wechat_user.set_session_key(Some(session_key.clone()));
    //解密获取 用户信息 组装数据
    let iv = sign.get("iv").unwrap();
    let encrypted_data = sign.get("encryptedData").unwrap();
    if let Ok(wx_info) = resolve_data(session_key.clone(), iv.clone(), encrypted_data.clone()) {
        openid = wx_info.openId.clone();
        wechat_user.set_nickname(Some(wx_info.nickName)); //昵称
        wechat_user.set_routine_openid(Some(wx_info.openId)); //设置openid
        wechat_user.set_headimgurl(Some(wx_info.avatarUrl)); //头像
        wechat_user.set_sex(Some(wx_info.gender)); //性别
        wechat_user.set_city(Some(wx_info.city)); //市
        wechat_user.set_country(Some(wx_info.country));
        wechat_user.set_province(Some(wx_info.province)); //省
        wechat_user.set_language(Some(wx_info.language)); //语言
    } else {
        return RespVO::<()>::from_error(&Error::from("获取会话密匙失败")).resp_json();
    }

    //openid为空则授权异常
    if openid.is_empty() {
        return RespVO::<()>::from_error(&Error::from("openid获取失败")).resp_json();
    }
    if unionid.is_empty() {
        wechat_user.set_unionid(None);
    }
    //新增或更新用户
    let uid = save_or_update_user(wechat_user).await;
    //根据用户信息 生成token
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let user = user_service.get(uid.to_string()).await.unwrap();
    let mut jwt_token = JWTToken::default();
    jwt_token.set_id(uid);
    jwt_token.set_super_admin(0);
    jwt_token.set_username(user.account().clone().unwrap_or_default());
    jwt_token.set_agency_code("".to_string());
    jwt_token.set_from("wxapp".to_string());
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let token_info = jwt_token.create_token(cassie_config.jwt_secret());

    match token_info {
        Ok(token) => {
            let mut result = ApiSignInVO::default();
            result.set_cache_key(cache_key);
            result.set_user(Some(user));
            result.set_access_token(token);
            return RespVO::from(&result).resp_json();
        }
        Err(_) => {
            return RespVO::<()>::from_error(&Error::from("获取用户访问token失败")).resp_json();
        }
    }
}

//小程序支付回调
pub async fn notify() {
    todo!("待开发")
}

pub fn init_router() -> Router {
    Router::new().route("/wechat/mp_auth", post(mp_auth))
}
