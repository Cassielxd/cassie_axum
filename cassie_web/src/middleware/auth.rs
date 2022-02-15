use cassie_common::error::Error;
use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

use crate::cici_casbin::casbin_service::CasbinVals;
use crate::cici_casbin::{CASBIN_CONTEXT, is_white_list_api};
use tracing::Instrument;
use cassie_common::{RespVO};
use crate::service::CONTEXT;
use axum::http::HeaderValue;
use crate::vo::jwt::JWTToken;
use crate::{RequestModel, REQUEST_CONTEXT};

/**
 *struct:Auth
 *desc:权限验证中间件 初始化 REQUEST_CONTEXT
 *author:String
 *email:348040933@qq.com
 */
pub struct Auth;

#[async_trait]
impl<B> FromRequest<B> for Auth
    where
        B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        /*获取验证的  casbin_service*/
        let mut service = CASBIN_CONTEXT.clone();
        /*获取method path */
        let action = req.method().clone().to_string();
        let path = req.uri().clone().to_string();
        /*获取token*/
        let value = HeaderValue::from_str("").unwrap();
        let headers = req.headers().unwrap();
        let token = headers.get("access_token").unwrap_or(&value);
        /*判断是否在白名单里 如果不在进行验证*/
        if !is_white_list_api(&path, &CONTEXT.config.admin_white_list_api) {
            let token_value = token.to_str().unwrap_or("");
            /*token为空提示登录*/
            if token_value.is_empty() {
                let resp: RespVO<String> = RespVO {
                    code: Some("-1".to_string()),
                    msg: Some(format!("请登录")),
                    data: None,
                };
                return Err(Error::E(
                    serde_json::json!(&resp).to_string(),
                ));
            }
            let resp: RespVO<String> = RespVO {
                code: Some("-1".to_string()),
                msg: Some(format!("没有权限")),
                data: None,
            };
            /*验证token有效性*/
            match checked_token(token_value).await {
                Ok(data) => {
                    let tls = REQUEST_CONTEXT.clone();
                    tls.get_or(|| RequestModel {
                        uid: data.id.clone(),
                        username: data.username.clone(),
                        agency_code: data.agency_code.clone(),
                        product_code: "".to_string(),
                    });
                    // 获取用户名和租户编码 进入下一步资源认证
                    let vals = CasbinVals {
                        username: data.username,
                        agency_code: Option::from(data.agency_code),
                    };
                    /*casbin 验证有效性 处理返回结果集*/
                    if service.call(path, action, vals).await {
                        return Ok(Self {});
                    } else {
                        return Err(Error::from(serde_json::json!(&resp).to_string()));
                    }
                }
                Err(e) => {
                    //401 http状态码会强制前端退出当前登陆状态
                    return Err(Error::from(
                        serde_json::json!(&resp).to_string(),
                    ));
                }
            }
        } else {
            return Ok(Self {});
        }
    }
}

/**
 *method:checked_token
 *desc:校验token是否有效，未过期
 *author:String
 *email:348040933@qq.com
 */
pub async fn checked_token(token: &str) -> Result<JWTToken, Error> {
    //check token alive
    let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match token {
        Ok(token) => {
            return Ok(token);
        }
        Err(e) => {
            return Err(cassie_common::error::Error::from(e.to_string()));
        }
    }
}