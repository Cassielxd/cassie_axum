use crate::{
    dto::asi_dto::AsiGroupColumnDTO, entity::PageData, request::AsiQuery,
    service::crud_service::CrudService, CONTEXT,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use cassie_common::{error::Error, RespVO};
use validator::Validate;

pub async fn page(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .asi_service
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
pub async fn list(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT.asi_service.asi_column.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn save(
    Path(group_id): Path<String>,
    Json(arg): Json<Vec<AsiGroupColumnDTO>>,
) -> impl IntoResponse {
    /*验证是否存在业务分类*/
    let gid = group_id;
    let group = CONTEXT.asi_service.get(gid).await;
    if !group.is_ok() {
        return RespVO::<()>::from_error(&Error::E("业务分类没有定义".to_string())).resp_json();
    }
    /*执行验证逻辑*/
    for dto in &arg {
        if let Err(e) = dto.validate() {
            return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
        }
    }
    /*执行保存逻辑*/
    CONTEXT
        .asi_service
        .asi_column
        .save_batch_colums(group.unwrap(), arg)
        .await;
    RespVO::from_result(&Ok("保存成功".to_string())).resp_json()
}
