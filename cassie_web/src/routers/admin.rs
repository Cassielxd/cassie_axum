use crate::admin::{sys_user_resource, sys_auth_resource};
use axum::{http::HeaderMap, Router, routing::{get, post}};

pub fn routers() -> Router {
    Router::new()
        .nest("/user", sys_user_resource::init_router())
        .route("/captcha/:uuid", get(sys_auth_resource::captcha_img))
        .route("/login", post(sys_auth_resource::login))
}