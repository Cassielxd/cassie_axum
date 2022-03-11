use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use cassie_common::{error::Error, RespVO};
use validator::Validate;

use crate::{
    
    entity::PageData,
    request::AsiQuery,
    service::crud_service::CrudService,
    CONTEXT, dto::asi_dto::AsiGroupValuesDTO,
};

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

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let dto = CONTEXT.asi_service.asi_column.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

pub async fn save(Json(arg): Json<Vec<AsiGroupValuesDTO>>) -> impl IntoResponse {
    /*执行验证逻辑*/
    for dto in &arg {
        if let Err(e) = dto.validate() {
            return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
        }
    }
    CONTEXT.asi_service.asi_values.save_batch_values(arg).await;
    RespVO::from_result(&Ok("保存成功".to_string())).resp_json()
}
