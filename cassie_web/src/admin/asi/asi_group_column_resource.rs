use crate::{
    service::{asi::asi_service::AsiGroupService, crud_service::CrudService},
    APPLICATION_CONTEXT,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use cassie_common::{error::Error, RespVO};
use cassie_domain::{dto::asi_dto::AsiGroupColumnDTO, entity::PageData, request::AsiQuery};
use validator::Validate;

pub async fn get_column_one(Path(id): Path<String>) -> impl IntoResponse {
    let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
    let res = asi_service.asi_column.get(id).await;
    RespVO::from_result(&res).resp_json()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
    asi_service.asi_column.del(&id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}

pub async fn page(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
    let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
    let arg = arg.unwrap();
    let vo = asi_service
        .asi_column
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
pub async fn list(
    Path(group_code): Path<String>,
    arg: Option<Query<AsiQuery>>,
) -> impl IntoResponse {
    let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
    let res = asi_service
        .asi_column
        .fetch_list_by_column("group_code", &vec![group_code])
        .await;
    RespVO::from_result(&res).resp_json()
}

pub async fn save(
    Path(group_code): Path<String>,
    Json(dto): Json<AsiGroupColumnDTO>,
) -> impl IntoResponse {
    let asi_service = APPLICATION_CONTEXT.get::<AsiGroupService>();
    /*验证是否存在业务分类*/
    let group = asi_service
        .fetch_list_by_column("group_code", &vec![group_code])
        .await;
    match group {
        Ok(list) => {
            let group_info = list.get(0);
            /*执行验证逻辑*/
            if let Err(e) = dto.validate() {
                return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
            }
            /*执行保存逻辑*/
            asi_service
                .asi_column
                .save_column(group_info.unwrap().clone(), dto)
                .await;
            RespVO::from_result(&Ok("保存成功".to_string())).resp_json()
        }
        Err(_) => {
            return RespVO::<()>::from_error(&Error::E("业务分类没有定义".to_string())).resp_json();
        }
    }
}
pub fn init_router() -> Router {
    Router::new()
        .route(
            "/asi/column/list/:group_code",
            get(list).post(save).put(save),
        )
        .route(
            "/asi/column/get_column_one/:id",
            get(get_column_one).delete(delete),
        )
}
