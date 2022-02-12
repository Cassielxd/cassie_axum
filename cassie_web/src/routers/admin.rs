use crate::admin::{sys_user_resource, sys_auth_resource, sys_role_resource,
                   sys_params_resource, sys_dict_type_resource, sys_dict_value_resource};
use axum::{Router, routing::{get, post}};

pub fn routers() -> Router {
    Router::new()
        .nest("/user", sys_user_resource::init_router())
        .nest("/role", sys_role_resource::init_router())
        .nest("/params", sys_params_resource::init_router())
        .nest("/dict/type", sys_dict_type_resource::init_router())
        .nest("/dict/value", sys_dict_value_resource::init_router())
        .route("/captcha/:uuid", get(sys_auth_resource::captcha_img))
        .route("/login", post(sys_auth_resource::login))
}