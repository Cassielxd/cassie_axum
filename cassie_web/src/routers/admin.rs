use crate::admin::sys_auth_resource;
use axum::{http::HeaderMap, Router, routing::{get, post}};

pub fn routers() -> Router {
    Router::new().route("/captcha/:uuid", get(sys_auth_resource::captcha_img)).route("/login", post(sys_auth_resource::login))
}