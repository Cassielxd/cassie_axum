use std::collections::HashMap;

use axum::{extract::Query, response::IntoResponse, routing::post, Json, Router};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_config::config::ApplicationConfig;
use cassie_domain::vo::{jwt::JWTToken, sign_in::ApiSignInVO, wx::WxSignInVo};

use crate::{
    service::{
        api::user_service::UserService, crud_service::CrudService,
        wechat::wxapp_service::{wxapp_auth, silence_auth_no_login},
    },
    APPLICATION_CONTEXT,
};

//小程序授权登录
pub async fn mp_auth(Json(sign): Json<WxSignInVo>) -> impl IntoResponse {
    match wxapp_auth(sign).await {
        Ok(uid) => {
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
            //创建token
            match jwt_token.create_token(cassie_config.jwt_secret()) {
                Ok(token) => {
                    let mut result = ApiSignInVO::default();
                    result.set_user(Some(user));
                    result.set_access_token(token);
                    return RespVO::from(&result).resp_json();
                }
                Err(_) => {
                    return RespVO::<()>::from_error(&Error::from("获取用户访问token失败"))
                        .resp_json();
                }
            }
        }
        Err(e) => {
            return RespVO::<()>::from_error(&e).resp_json();
        }
    }
}
//静默授权 不登录
pub async fn silence_auth(arg: Query<HashMap<String, String>>) {
    let params = arg.0;
    let code = params.get("code").unwrap();
    match silence_auth_no_login(code.clone()).await{
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

//小程序支付回调
pub async fn notify() {
    todo!("待开发")
}

pub fn init_router() -> Router {
    Router::new().route("/wechat/mp_auth", post(mp_auth))
}
