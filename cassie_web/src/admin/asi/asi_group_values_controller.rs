use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use cassie_common::{error::Error, RespVO};
use validator::Validate;

use crate::{
    dto::asi_dto::AsiGroupValuesDTO, request::AsiQuery,
    service::crud_service::CrudService, CONTEXT,
};

pub async fn list(
    Path(group_code): Path<String>,
    arg: Option<Query<AsiQuery>>,
) -> impl IntoResponse {
    let mut arg = arg.unwrap();
    arg.0.group_code=Option::Some(group_code);
    let vo = CONTEXT.asi_service.asi_column.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn save(
    Path(group_code): Path<String>,
    Json(arg): Json<Vec<AsiGroupValuesDTO>>,
) -> impl IntoResponse {
    /*执行验证逻辑*/
    for dto in &arg {
        if let Err(e) = dto.validate() {
            return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
        }
    }
    let res = CONTEXT.asi_service.save_values(group_code, arg).await;
    RespVO::from_result(&res).resp_json()
}
