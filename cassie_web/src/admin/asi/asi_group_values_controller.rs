use axum::{extract::{Query, Path}, response::IntoResponse, Json};
use cassie_common::{RespVO, error::Error};

use crate::{request::AsiQuery, CONTEXT, entity::PageData, service::crud_service::CrudService, dto::asi_dto::{ AsiGroupColumnDTO, AsiGroupValuesDTO}};

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

pub async fn save(Json(arg): Json<Vec<AsiGroupValuesDTO>>) -> impl IntoResponse {

    CONTEXT
       .asi_service.asi_values
       .save_batch_values(arg)
       .await;
   RespVO::from_result(&Ok("保存成功".to_string())).resp_json()
}
