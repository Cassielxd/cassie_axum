use crate::cici_casbin::is_white_api_list_api;
use crate::APPLICATION_CONTEXT;
use axum::http::HeaderValue;
use axum::{
  async_trait,
  extract::{FromRequest, RequestParts},
};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_config::config::ApplicationConfig;

use super::{checked_token, set_local};

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
    let resp: RespVO<String> = RespVO {
      code: Some(-1),
      msg: Some(format!("请登录")),
      data: None,
    };

    /*判断是否在白名单里 如果不在进行验证*/
    if !is_white_api_list_api(path.clone()) {
      let token_value = token.to_str().unwrap_or("");
      /*token为空提示登录*/
      if token_value.is_empty() {
        return Err(Error::E(serde_json::json!(&resp).to_string()));
      }
      /*验证token有效性*/
      match checked_token(token_value).await {
        Ok(data) => {
          //登录了但是不需要权限
          let data1 = data.clone();
          set_local(data1, path.clone());
          return Ok(Self {});
        }
        Err(e) => {
          println!("error:{}", e);
          //401 http状态码会强制前端退出当前登陆状态
          return Err(Error::from(serde_json::json!(&resp).to_string()));
        }
      }
    } else {
      return Ok(Self {});
    }
  }
}
