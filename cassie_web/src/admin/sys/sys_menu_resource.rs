use std::collections::HashMap;

use crate::middleware::get_local;
use crate::observe::event::{CassieEvent, CustomEvent};
use crate::service::crud_service::CrudService;
use crate::service::fire_event;
use crate::service::sys_menu_service::{get_user_menu_list, SysMenuService};
use crate::{fire_script_event, APPLICATION_CONTEXT};
use axum::routing::get;
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json, Router,
};
use cassie_common::RespVO;
use cassie_domain::dto::sys_menu_dto::SysMenuDTO;
use cassie_domain::entity::PageData;
use cassie_domain::request::SysMenuQuery;

/**
 *method:/menu
 *desc:菜单服务 分页查询
 *author:String
 *email:348040933@qq.com
 */

pub async fn page(arg: Option<Query<SysMenuQuery>>) -> impl IntoResponse {
    let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
    let arg = arg.unwrap();
    let vo = sys_menu_service
        .page(
            &arg,
            PageData {
                page_no: arg.page().clone(),
                page_size: arg.limit().clone(),
            },
        )
        .await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn list() -> impl IntoResponse {
    let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
    let vo = sys_menu_service.menu_list().await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn nav() -> impl IntoResponse {
    let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
    let request_model = get_local().unwrap();
    let vo = get_user_menu_list(request_model.uid().clone().to_string(), request_model.super_admin().clone(), request_model.agency_code().clone()).await;
    //事件测试代码
    fire_script_event(None, None).await;
    RespVO::from_result(&vo).resp_json()
}

/**
 *method:get_by_id
 *desc:菜单服务id获取
 *author:String
 *email:348040933@qq.com
 */
pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
    let dto = sys_menu_service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}
pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
    sys_menu_service.del(&id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}

/**
 *method:save
 *desc:菜单服务保存
 *author:String
 *email:348040933@qq.com
 */

pub async fn save(Json(arg): Json<SysMenuDTO>) -> impl IntoResponse {
    let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
    sys_menu_service.save_or_update(arg).await;
    RespVO::from(&"更新成功".to_string()).resp_json()
}

pub fn init_router() -> Router {
    Router::new()
        .route("/menu/list", get(list))
        .route("/menu/nav", get(nav))
        .route("/menu/:id", get(get_by_id).delete(delete))
        .route("/menu", get(page).post(save).put(save))
}
