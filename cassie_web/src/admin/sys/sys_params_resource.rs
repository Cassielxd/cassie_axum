use crate::dto::sys_params_dto::SysParamsDTO;
use crate::request::SysParamsQuery;
use crate::{entity::PageData, service::crud_service::CrudService, CONTEXT};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::Json;
use cassie_common::RespVO;

pub async fn page(arg: Option<Query<SysParamsQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .sys_params_service
        .page(
            &arg,
            PageData {
                page_no: arg.page.clone(),
                page_size: arg.limit.clone(),
            },
        )
        .await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn list(arg: Option<Query<SysParamsQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT.sys_params_service.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let dto = CONTEXT.sys_params_service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    CONTEXT.sys_params_service.del(&id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}

pub async fn save(Json(arg): Json<SysParamsDTO>) -> impl IntoResponse {
    let mut entity = arg.into();
    let vo = CONTEXT.sys_params_service.save(&mut entity).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn edit(Json(arg): Json<SysParamsDTO>) -> impl IntoResponse {
    let id = arg.id.clone();
    let mut entity = arg.into();
    CONTEXT.sys_params_service.update_by_id(id.unwrap().to_string(),&mut entity).await;
    RespVO::from(&"更新成功".to_string()).resp_json()
}