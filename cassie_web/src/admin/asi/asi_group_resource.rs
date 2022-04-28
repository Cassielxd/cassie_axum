use crate::service::asi::asi_service::AsiGroupService;
use crate::service::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use axum::{
  extract::{Path, Query},
  response::IntoResponse,
  routing::get,
  Json, Router,
};
use cassie_common::RespVO;
use cassie_domain::{dto::asi_dto::AsiGroupDTO, request::AsiQuery};
pub async fn page(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
  let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
  let arg = arg.unwrap();
  let vo = asi_service.get_group_page(arg.0).await;
  RespVO::from_result(&vo).resp_json()
}
pub async fn list(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
  let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
  let arg = arg.unwrap();
  let vo = asi_service.get_group_list(arg.0).await;
  RespVO::from_result(&vo).resp_json()
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
  let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
  let dto = asi_service.get(id).await;
  RespVO::from_result(&dto).resp_json()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
  let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
  asi_service.del(&id).await;
  RespVO::from(&"删除成功".to_string()).resp_json()
}

pub async fn save(Json(arg): Json<AsiGroupDTO>) -> impl IntoResponse {
  let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
  let res = asi_service.save_group(arg).await;
  RespVO::from_result(&res).resp_json()
}

pub async fn edit(Json(arg): Json<AsiGroupDTO>) -> impl IntoResponse {
  let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();

  let id = arg.id().clone();
  asi_service
    .update_by_id(id.unwrap().to_string(), &mut arg.into())
    .await;
  RespVO::from(&"更新成功".to_string()).resp_json()
}
pub fn init_router() -> Router {
  Router::new()
    .route("/asi/group", get(page).post(save).put(edit))
    .route("/asi/group/:id", get(get_by_id).delete(delete))
    .route("/asi/group/list", get(list))
}
