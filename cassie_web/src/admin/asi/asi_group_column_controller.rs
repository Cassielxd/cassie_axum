use axum::{extract::{Query, Path}, response::IntoResponse, Json};
use cassie_common::{RespVO, error::Error};
use validator::Validate;
use crate::{request::AsiQuery, CONTEXT, entity::PageData, service::crud_service::CrudService, dto::asi_dto::{ AsiGroupColumnDTO}};

pub async fn page(arg:  Option<Query<AsiQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .asi_service.asi_column
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
    let dto = CONTEXT.asi_service.asi_column.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

pub async fn save(Path(group_id): Path<String>,Json(arg): Json<Vec<AsiGroupColumnDTO>>) -> impl IntoResponse {
    /*验证是否存在业务分类*/
    let gid = group_id;
    let group = CONTEXT.asi_service.get(gid).await;
    if !group.is_ok(){
        return RespVO::<()>::from_error(&Error::E("业务分类没有定义".to_string())).resp_json();
    }
    /*执行验证逻辑*/
    for dto in &arg {
        if let Err(e) = dto.validate() {
            return RespVO::<()>::from_error( &Error::E(e.to_string())).resp_json();
        }
    }
    /*执行保存逻辑*/
     CONTEXT
        .asi_service.asi_column
        .save_batch_colums(group.unwrap(),arg)
        .await;
    RespVO::from_result(&Ok("保存成功".to_string())).resp_json()
}
