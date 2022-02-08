use cassie_common::error::Error;
use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

use crate::{casbin_adapter::CASBIN_CONTEXT, cici_casbin::casbin_service::CasbinVals};

pub struct Auth {}
#[async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = Error;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers().unwrap();
        let mut service = CASBIN_CONTEXT.service.clone();
        let action = req.method().clone().;
        let path = req.uri().clone().to_string();

        let cas = CasbinVals {
            subject: "lixingdong".to_string(),
            domain: Option::from("rivet_admin".to_string()),
        };
        let result = service.call(path, action, cas).await;
        if let Ok(b) = result {
            println!("认证{}",b);
            if b {
                return Ok(Self {});
            } else {
                Err(Error::from("没有权限"))
            }
        } else {
            Err(Error::from("参数异常"))
        }
    }
}
