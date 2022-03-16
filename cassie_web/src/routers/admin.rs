use crate::admin::{
    asi::{asi_group_column_controller, asi_group_controller, asi_group_values_controller},
    sys::{
        sys_auth_resource, sys_dict_type_resource, sys_dict_value_resource, sys_menu_resource,
        sys_params_resource, sys_role_resource, sys_user_resource,
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
        .route("/menu/list", get(sys_menu_resource::list))
        .route("/menu/nav", get(sys_menu_resource::nav))
        .route(
            "/menu/:id",
            get(sys_menu_resource::get_by_id).delete(sys_menu_resource::delete),
        )
        .route(
            "/menu",
            get(sys_menu_resource::page)
                .post(sys_menu_resource::save)
                .put(sys_menu_resource::save),
        )
        //-------------------------------------用户服务-------------------------------------------------------
        .route(
            "/user",
            get(sys_user_resource::page).post(sys_user_resource::save),
        )
        .route("/user/info", get(sys_user_resource::info))
        .route("/user/list", get(sys_user_resource::list))
        .route(
            "/user/:id",
            get(sys_user_resource::get_user_by_id).delete(sys_user_resource::delete),
        )
        //-------------------------------------角色服务-------------------------------------------------------
        .route(
            "/role",
            get(sys_role_resource::page)
                .post(sys_role_resource::save)
                .put(sys_role_resource::save),
        )
        .route(
            "/role/:id",
            get(sys_role_resource::get_by_id).delete(sys_role_resource::delete),
        )
        .route("/role/list", get(sys_role_resource::list))
        //-------------------------------------参数服务-------------------------------------------------------
        .route(
            "/params",
            get(sys_params_resource::page)
                .post(sys_params_resource::save)
                .put(sys_params_resource::edit),
        )
        .route("/params/list", get(sys_params_resource::list))
        .route(
            "/params/:id",
            get(sys_params_resource::get_by_id).delete(sys_params_resource::delete),
        )
        //-------------------------------------字典服务-------------------------------------------------------
        .route("/dict/type/all", get(sys_dict_type_resource::all))
        .route(
            "/dict/type",
            get(sys_dict_type_resource::page)
                .post(sys_dict_type_resource::save)
                .put(sys_dict_type_resource::edit),
        )
        .route(
            "/dict/type/:id",
            get(sys_dict_type_resource::get_by_id).delete(sys_dict_type_resource::delete),
        )
        .route(
            "/dict/value",
            get(sys_dict_value_resource::page)
                .post(sys_dict_value_resource::save)
                .put(sys_dict_value_resource::edit),
        )
        .route(
            "/dict/value/:id",
            get(sys_dict_value_resource::get_by_id).delete(sys_dict_value_resource::delete),
        )
        //-------------------------------------动态表单分组服务-------------------------------------------------------
        .route(
            "/asi/group",
            get(asi_group_controller::page)
                .post(asi_group_controller::save)
                .put(asi_group_controller::edit),
        )
        .route(
            "/asi/group/:id",
            get(asi_group_controller::get_by_id).delete(asi_group_controller::delete),
        )
        //-------------------------------------动态表单column服务-------------------------------------------------------
        .route("/asi/column", get(asi_group_column_controller::page))
        .route(
            "/asi/column/list",
            get(asi_group_column_controller::list)
                .post(asi_group_column_controller::save)
                .put(asi_group_column_controller::save),
        )
        //-------------------------------------动态表单value服务-------------------------------------------------------
        .route(
            "/asi/values/:group_code",
            get(asi_group_values_controller::list).post(asi_group_values_controller::save),
        )
}
