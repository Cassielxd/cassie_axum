use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use cassie_common::{error::Error, RespVO};

use crate::{ request::AsiQuery,
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

pub async fn save_from(
    Path(id): Path<String>,
    Json(arg): Json<HashMap<String,HashMap<String,String>>>,
) -> impl IntoResponse {
    /*执行验证逻辑*/
    let res = CONTEXT.asi_service.save_values_for_from(id, arg).await;
    RespVO::from_result(&res).resp_json()
}

pub async fn save_table(
    Path(id): Path<String>,
    Json(arg): Json<HashMap<String,Vec<HashMap<String,String>>>>,
) -> impl IntoResponse {
    /*执行验证逻辑*/
    let res = CONTEXT.asi_service.save_values_for_table(id, arg).await;
    RespVO::from_result(&res).resp_json()
}
