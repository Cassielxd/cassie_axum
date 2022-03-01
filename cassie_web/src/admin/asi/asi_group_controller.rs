use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use cassie_common::RespVO;

use crate::{
    dto::asi_dto::AsiGroupDTO, entity::PageData, request::AsiQuery,
    service::crud_service::CrudService, CONTEXT,
};

pub async fn page(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .asi_service
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

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let dto = CONTEXT.asi_service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

pub async fn save(Json(arg): Json<AsiGroupDTO>) -> impl IntoResponse {
    CONTEXT.asi_service.save(&mut arg.into()).await;
    RespVO::from(&"更新成功".to_string()).resp_json()
}
