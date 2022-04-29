use crate::service::{crud_service::CrudService, sys_config_service::SysConfigService};
use crate::APPLICATION_CONTEXT;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use cassie_common::{error::Error, RespVO};
use cassie_domain::{dto::sys_config_dto::SysConfigDTO, entity::PageData, request::SysConfigQuery};

pub async fn page(arg: Option<Query<SysConfigQuery>>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<SysConfigService>();
    let arg = arg.unwrap();
    let vo = service
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

pub async fn list(arg: Option<Query<SysConfigQuery>>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<SysConfigService>();
    let arg = arg.unwrap();
    let vo = service.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<SysConfigService>();
    let dto = service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<SysConfigService>();
    service.del(&id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}

pub async fn save(Json(arg): Json<SysConfigDTO>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<SysConfigService>();
    let mut entity = arg.into();
    let vo = service.save(&mut entity).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn edit(Json(arg): Json<SysConfigDTO>) -> impl IntoResponse {
    let service = APPLICATION_CONTEXT.get::<SysConfigService>();
    let id = arg.id().clone();
    let mut entity = arg.into();
    service.update_by_id(id.unwrap().to_string(), &mut entity).await;
    RespVO::from(&"更新成功".to_string()).resp_json()
}

pub fn init_router() -> Router {
    Router::new()
        .route("/sys_config", get(page).post(save).put(edit))
        .route("/sys_config/list", get(list))
        .route("/sys_config/:id", get(get_by_id).delete(delete))
}
