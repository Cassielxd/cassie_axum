use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use cassie_common::RespVO;

use crate::{
    dto::asi_dto::AsiGroupDTO, entity::PageData, request::AsiQuery,
    service::crud_service::CrudService, CONTEXT,
};

pub async fn page(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT.asi_service.get_group(arg.0).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let dto = CONTEXT.asi_service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    CONTEXT.asi_service.del(&id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}

pub async fn save(Json(arg): Json<AsiGroupDTO>) -> impl IntoResponse {
    CONTEXT.asi_service.save(&mut arg.into()).await;
    RespVO::from(&"保存成功".to_string()).resp_json()
}

pub async fn edit(Json(arg): Json<AsiGroupDTO>) -> impl IntoResponse {
    let id = arg.id.clone();
    CONTEXT
        .asi_service
        .update_by_id(id.unwrap().to_string(), &mut arg.into())
        .await;
    RespVO::from(&"更新成功".to_string()).resp_json()
}
pub fn init_router() -> Router {
    Router::new()
        .route("/asi/group", get(page).post(save).put(edit))
        .route("/asi/group/:id", get(get_by_id).delete(delete))
}
