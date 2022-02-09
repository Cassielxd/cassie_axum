use crate::dto::sign_in::{SignInDTO, CatpchaDTO};
use cassie_common::RespVO;
use cassie_common::error::Error;
use validator::Validate;
use axum::response::{Response, IntoResponse};
use axum::Json;
use axum::body::Body;
use crate::service::CONTEXT;
use std::time::Duration;
use captcha::Captcha;
use captcha::filters::{Noise, Wave, Dots};
use axum::extract::{Query, Path};
use cassie_common::utils::string::IsEmpty;


pub async fn login(Json(sign): Json<SignInDTO>) -> impl IntoResponse {
    if let Err(e) = sign.validate() {
        return RespVO::<()>::from_error("-1", &Error::E(e.to_string())).resp_json();
    }
    if let Ok(code) = CONTEXT.cache_service.get_string(&format!("captch:uuid_{}", &sign.uuid.clone().unwrap())).await {
        if !code.eq(&sign.vcode.clone().unwrap()) {
            return RespVO::<()>::from_error("-1", &Error::E("验证码错误".to_string())).resp_json();
        }
    }
    let vo = CONTEXT.sys_auth_service.sign_in(&sign).await;
    return RespVO::from_result(&vo).resp_json();
}

pub async fn captcha_img(Path(uuid): Path<String>) -> Response<Body> {
    if uuid.is_empty() {
        return RespVO::<()>::from_error("-1", &Error::from("uuid不能为空!")).resp_json();
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
    if !uuid.is_empty() {
        async_std::task::block_on(async {
            CONTEXT
                .cache_service
                .set_string_ex(
                    &format!("captch:uuid_{}", uuid.clone()),
                    captcha_str.as_str(),
                    Some(Duration::from_secs(180)),
                ).await;
        });
    }
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "no-cache")
        .header("Content-Type", "image/png")
        .body(Body::from(png))
        .unwrap()
}