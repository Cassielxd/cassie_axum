use crate::service::cache_service::CacheService;
use crate::service::sys_auth_service::SysAuthService;
use crate::APPLICATION_CONTEXT;

use axum::body::Body;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Json;
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_domain::dto::sign_in::SignInDTO;
use validator::Validate;

pub async fn login(Json(sign): Json<SignInDTO>) -> impl IntoResponse {
  let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
  let sys_auth_service = APPLICATION_CONTEXT.get::<SysAuthService>();
  if let Err(e) = sign.validate() {
    return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
  }
  if let Ok(code) = cache_service
    .get_string(&format!("_captch:uuid_{}", &sign.uuid().clone().unwrap()))
    .await
  {
    if !code.eq(&sign.vcode().clone().unwrap()) {
      return RespVO::<()>::from_error(&Error::E("验证码错误".to_string()))
        .resp_json();
    }
  }
  let vo = sys_auth_service.sign_in(&sign).await;

  return RespVO::from_result(&vo).resp_json();
}

pub async fn captcha_img(Path(uuid): Path<String>) -> Response<Body> {
  let cache_service = APPLICATION_CONTEXT.get::<CacheService>();
  if uuid.is_empty() {
    return RespVO::<()>::from_error(&Error::from("uuid不能为空!")).resp_json();
  }
  let mut captcha = Captcha::new();
  captcha
    .add_chars(4)
    .apply_filter(Noise::new(0.1))
    .apply_filter(Wave::new(1.0, 10.0).horizontal())
    // .apply_filter(Wave::new(2.0, 20.0).vertical())
    .view(160, 60)
    .apply_filter(Dots::new(4));
  let png = captcha.as_png().unwrap();
  let captcha_str = captcha.chars_as_string().to_lowercase();
  async_std::task::block_on(async {
    let res = cache_service
      .set_string_ex(
        &format!("_captch:uuid_{}", uuid.clone()),
        captcha_str.as_str(),
        Some(std::time::Duration::from_secs(60 * 5)),
      )
      .await;
    println!("{:?}", res);
  });
  Response::builder()
    .header("Access-Control-Allow-Origin", "*")
    .header("Cache-Control", "no-cache")
    .header("Content-Type", "image/png")
    .body(Body::from(png))
    .unwrap()
}
