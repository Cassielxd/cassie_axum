use std::sync::{Arc, Mutex};

use crate::cici_casbin::casbin_service::{CasbinService, CasbinVals};
use crate::cici_casbin::is_auth_list_api;
use crate::{cici_casbin::is_white_list_api, APPLICATION_CONTEXT};
use axum::http::HeaderValue;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_config::config::ApplicationConfig;
use cassie_domain::request::RequestModel;
use cassie_domain::vo::jwt::JWTToken;

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
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();

        /*获取method path */
        let action = req.method().clone().to_string();
        let path = req.uri().path().to_string();

        /*获取token*/
        let value = HeaderValue::from_str("").unwrap();
        let headers = req.headers().unwrap();
        let token = headers.get("access_token").unwrap_or(&value);
        let mut resp: RespVO<String> = RespVO {
            code: Some(-1),
            msg: Some(format!("请登录")),
            data: None,
        };

        /*判断是否在白名单里 如果不在进行验证*/
        if !is_white_list_api(path.clone()) {
            let token_value = token.to_str().unwrap_or("");
            /*token为空提示登录*/
            if token_value.is_empty() {
                return Err(Error::E(serde_json::json!(&resp).to_string()));
            }
            resp.msg = Some(format!("没有权限"));
            /*验证token有效性*/
            match checked_token(token_value).await {
                Ok(data) => {
                    //登录了但是不需要权限
                    let data1 = data.clone();
                    set_local(data1);
                    // 获取用户名和租户编码 进入下一步资源认证
                    let vals = CasbinVals {
                        uid: data.id().to_string(),
                        agency_code: Option::from(data.agency_code().clone()),
                    };
                    /*获取验证的  casbin_service*/
                    let service = APPLICATION_CONTEXT.get::<CasbinService>();
                    /*casbin 验证有效性 处理返回结果集*/

                    if is_auth_list_api(path.clone()) || service.call(path, action, vals).await {
                        return Ok(Self {});
                    } else {
                        return Err(Error::from(serde_json::json!(&resp).to_string()));
                    }
                }
                Err(e) => {
                    //401 http状态码会强制前端退出当前登陆状态
                    return Err(Error::from(serde_json::json!(&resp).to_string()));
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
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let token = JWTToken::verify(cassie_config.jwt_secret(), token);
    token
}

pub fn get_local()->Option<RequestModel>{
    let request_model = APPLICATION_CONTEXT.try_get_local::<Arc<Mutex<RequestModel>>>();
    match request_model {
        None => {
            None
        }
        Some(e) => {
            let  e = e.lock().unwrap();
            let mut model = RequestModel::default();
            model.set_uid(e.uid().clone());
            model.set_agency_code(e.agency_code().clone());
            model.set_super_admin(e.super_admin().clone());
            model.set_username(e.username().clone());
            Some(model)
        }
    }
}

pub fn set_local(data: JWTToken) {
    let request_model = APPLICATION_CONTEXT.try_get_local::<Arc<Mutex<RequestModel>>>();
    match request_model {
        Some(d) => {
            let a = d.clone();
            let mut model = a.lock().unwrap();
            model.set_uid(data.id().clone());
            model.set_agency_code(data.agency_code().clone());
            model.set_super_admin(data.super_admin().clone());
            model.set_username(data.username().clone());
        }
        None => {
            APPLICATION_CONTEXT.set_local(move || {
                let mut model = RequestModel::default();
                model.set_uid(data.id().clone());
                model.set_agency_code(data.agency_code().clone());
                model.set_super_admin(data.super_admin().clone());
                model.set_username(data.username().clone());
                let mutex = Arc::new(Mutex::new(model));
                return mutex;
            });
        }
    }
}
