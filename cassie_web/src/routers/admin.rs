use crate::admin::{
    asi::{asi_group_column_resource, asi_group_resource, asi_group_values_resource},
    sys::{
        sys_auth_resource, sys_dict_type_resource, sys_dict_value_resource, sys_menu_resource,
        sys_params_resource, sys_role_resource, sys_upload_resource, sys_user_resource,
    },
};
use axum::{
    routing::{get, post},
    Router,
};
pub fn routers() -> Router {
    Router::new()
        //-------------------------------------登录服务-------------------------------------------------------
        .route("/captcha/:uuid", get(sys_auth_resource::captcha_img))
        .route("/login", post(sys_auth_resource::login))
        //-------------------------------------菜单服务-------------------------------------------------------
        .merge(sys_menu_resource::init_router())
        //-------------------------------------用户服务-------------------------------------------------------
        .merge(sys_user_resource::init_router())
        //-------------------------------------角色服务-------------------------------------------------------
        .merge(sys_role_resource::init_router())
        //-------------------------------------参数服务-------------------------------------------------------
        .merge(sys_params_resource::init_router())
        //-------------------------------------字典服务-------------------------------------------------------
        .merge(sys_dict_type_resource::init_router())
        .merge(sys_dict_value_resource::init_router())
        //-------------------------------------动态表单分组服务-------------------------------------------------------
        .merge(asi_group_resource::init_router())
        //-------------------------------------动态表单column服务-------------------------------------------------------
        .merge(asi_group_column_resource::init_router())
        //-------------------------------------动态表单value服务-------------------------------------------------------
        .merge(asi_group_values_resource::init_router())
        //-------------------------------------upload服务-------------------------------------------------------
        .merge(sys_upload_resource::init_router())
}
