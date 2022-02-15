use crate::admin::{sys_user_resource, sys_auth_resource, sys_role_resource,
                   sys_params_resource, sys_dict_type_resource, sys_dict_value_resource};
use axum::{Router, routing::{get, post}};

pub fn routers() -> Router {
    Router::new()
        //-------------------------------------用户服务-------------------------------------------------------
        .route("/user", get(sys_user_resource::page))
        .route("/user/list", get(sys_user_resource::list))
        .route("/user/save", post(sys_user_resource::save))
        .route("/user/:id", get(sys_user_resource::get_user_by_id))
        //-------------------------------------角色服务-------------------------------------------------------
        .route("/role", get(sys_role_resource::page))
        .route("/role/save", post(sys_role_resource::save))
        .route("/role/casbin_test", get(sys_role_resource::casbin_test))
        //-------------------------------------参数服务-------------------------------------------------------
        .route("/params", get(sys_params_resource::page))
        .route("/params/list", get(sys_params_resource::list))
        .route("/params/save", post(sys_params_resource::save))
        .route("/params/:id", get(sys_params_resource::get_by_id))
        //-------------------------------------字典服务-------------------------------------------------------
        .route("/dict/type", get(sys_dict_type_resource::page))
        .route("/dict/type/save", post(sys_dict_type_resource::save))
        .route("/dict/type/:id", get(sys_dict_type_resource::get_by_id))
        .route("/dict/value", get(sys_dict_value_resource::page))
        .route("/dict/value/save", post(sys_dict_value_resource::save))
        .route("/dict/value/:id", get(sys_dict_value_resource::get_by_id))
        //-------------------------------------登录服务-------------------------------------------------------
        .route("/captcha/:uuid", get(sys_auth_resource::captcha_img))
        .route("/login", post(sys_auth_resource::login))
}