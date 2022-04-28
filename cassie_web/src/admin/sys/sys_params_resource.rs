use crate::service::crud_service::CrudService;
use crate::service::sys_params_service::SysParamsService;
use crate::APPLICATION_CONTEXT;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use cassie_common::RespVO;
use cassie_domain::dto::sys_params_dto::SysParamsDTO;
use cassie_domain::entity::PageData;
use cassie_domain::request::SysParamsQuery;

pub async fn page(arg: Option<Query<SysParamsQuery>>) -> impl IntoResponse {
  let sys_params_service = APPLICATION_CONTEXT.get::<SysParamsService>();
  let arg = arg.unwrap();
  let vo = sys_params_service
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

pub async fn list(arg: Option<Query<SysParamsQuery>>) -> impl IntoResponse {
  let sys_params_service = APPLICATION_CONTEXT.get::<SysParamsService>();
  let arg = arg.unwrap();
  let vo = sys_params_service.list(&arg).await;
  RespVO::from_result(&vo).resp_json()
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
  let sys_params_service = APPLICATION_CONTEXT.get::<SysParamsService>();
  let dto = sys_params_service.get(id).await;
  RespVO::from_result(&dto).resp_json()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
  let sys_params_service = APPLICATION_CONTEXT.get::<SysParamsService>();
  sys_params_service.del(&id).await;
  RespVO::from(&"删除成功".to_string()).resp_json()
}

pub async fn save(Json(arg): Json<SysParamsDTO>) -> impl IntoResponse {
  let sys_params_service = APPLICATION_CONTEXT.get::<SysParamsService>();
  let mut entity = arg.into();
  let vo = sys_params_service.save(&mut entity).await;
  RespVO::from_result(&vo).resp_json()
}

pub async fn edit(Json(arg): Json<SysParamsDTO>) -> impl IntoResponse {
  let sys_params_service = APPLICATION_CONTEXT.get::<SysParamsService>();
  let id = arg.id().clone();
  let mut entity = arg.into();
  sys_params_service
    .update_by_id(id.unwrap().to_string(), &mut entity)
    .await;
  RespVO::from(&"更新成功".to_string()).resp_json()
}

pub fn init_router() -> Router {
  Router::new()
    .route("/params", get(page).post(save).put(edit))
    .route("/params/list", get(list))
    .route("/params/:id", get(get_by_id).delete(delete))
}
