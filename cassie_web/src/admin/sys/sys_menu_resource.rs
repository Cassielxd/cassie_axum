use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use cassie_common::RespVO;

use crate::{
    dto::sys_menu_dto::SysMenuDTO, entity::PageData, request::SysMenuQuery,
    service::crud_service::CrudService, CONTEXT,
};

/**
 *method:/menu
 *desc:菜单服务 分页查询
 *author:String
 *email:348040933@qq.com
 */

pub async fn page(arg: Option<Query<SysMenuQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .sys_menu_service
        .page(
            &arg,
            PageData {
                page_no: arg.page_no.clone(),
                page_size: arg.page_size.clone(),
            },
        )
        .await;
    RespVO::from_result(&vo).resp_json()
}

/**
 *method:get_by_id
 *desc:菜单服务id获取
 *author:String
 *email:348040933@qq.com
 */
pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let dto = CONTEXT.sys_menu_service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

/**
 *method:save
 *desc:菜单服务保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysMenuDTO>) -> impl IntoResponse {
    CONTEXT.sys_menu_service.save_or_update(arg).await;
    RespVO::from(&"更新成功".to_string()).resp_json()
}
