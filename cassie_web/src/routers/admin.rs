use crate::{
    admin::{
        asi::{asi_group_column_resource, asi_group_resource, asi_group_values_resource},
        sys::{
            sys_auth_resource, sys_config_resource, sys_config_tab_resource, sys_dict_type_resource, sys_dict_value_resource, sys_group_data_resource, sys_group_resource, sys_menu_resource,
            sys_params_resource, sys_role_resource, sys_upload_resource, sys_user_resource,
        }, jsruntime::init_router,
    },
    middleware::{auth_admin, event::EventMiddleware},
};
use axum::{
    middleware::from_extractor,
    routing::{get, post},
    Router,
};
use tower::layer::layer_fn;
pub fn routers() -> Router {
    need_auth_routers().merge(noneed_auth_routers())
}
//需要权限认证的路由
pub fn need_auth_routers() -> Router {
    Router::new()
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
        //-------------------------------------系统配置-------------------------------------------------------
        .merge(sys_config_resource::init_router())
        .merge(sys_config_tab_resource::init_router())
        //-------------------------------------组合数据-------------------------------------------------------
        .merge(sys_group_resource::init_router())
        .merge(sys_group_data_resource::init_router())

        .merge(init_router())
        .layer(layer_fn(|inner| EventMiddleware { inner })) //第二执行的
        .layer(from_extractor::<auth_admin::Auth>()) //最先执行的
}
//不需要权限认证的路由
pub fn noneed_auth_routers() -> Router {
    Router::new()
        //-------------------------------------登录服务-------------------------------------------------------
        .route("/captcha/:uuid", get(sys_auth_resource::captcha_img))
        .route("/login", post(sys_auth_resource::login))
}
