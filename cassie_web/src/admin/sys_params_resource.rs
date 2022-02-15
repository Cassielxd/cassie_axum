use crate::{
    dto::sys_user_dto::SysUserDTO,
    entity::PageData,
    request::SysUserQuery,
    service::{crud_service::CrudService, CONTEXT},
};
use cassie_common::RespVO;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use axum::routing::post;
use cassie_common::error::Error;
use validator::Validate;
use axum::extract::{Path, Query};
use crate::request::SysParamsQuery;
use crate::dto::sys_params_dto::SysParamsDTO;

pub async fn page(arg: Option<Query<SysParamsQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .sys_params_service
        .page(
            &arg,
            PageData {
                page_no: arg.0.page_no.clone(),
                page_size: arg.0.page_size.clone(),
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

pub async fn save(Json(arg): Json<SysParamsDTO>) -> impl IntoResponse {
    let mut entity = arg.into();
    let vo = CONTEXT.sys_params_service.save(&mut entity).await;
    RespVO::from_result(&vo).resp_json()
}

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(page))
        .route("/list", get(list))
        .route("/save", post(save))
        .route("/:id", get(get_by_id))
}