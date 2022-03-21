use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json, Router, routing::{post, get},
};
use cassie_common::{error::Error, RespVO};

use crate::{ request::AsiQuery,
    service::crud_service::CrudService, CONTEXT,
};

pub async fn list(
    Path(id): Path<String>,
    arg: Option<Query<AsiQuery>>,
) -> impl IntoResponse {
    let  arg = arg.unwrap();
    let vo = CONTEXT.asi_service.list(&arg).await;
    if let Ok(r)= vo {
          let group = r.get(0);
         let r = CONTEXT.asi_service.value_list(&id,&group.unwrap()).await;
        return RespVO::from_result(&r).resp_json();
    }
     RespVO::<()>::from_error(&Error::E("业务分类没有定义".to_string())).resp_json()
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

pub fn init_router()->Router{
    Router::new().route(
        "/asi/values/:id/from",
        post(save_from),
    ).route(
        "/asi/values/:id/table",
        post(save_table),
    ).route(
        "/asi/values/:id/",
        get(list),
    )
}