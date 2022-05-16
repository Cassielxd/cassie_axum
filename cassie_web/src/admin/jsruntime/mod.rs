use crate::initialize::rules::init;
use crate::service::ops::{get_msg, clear_msg};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use cassie_common::error::Error;
use cassie_common::RespVO;
use std::collections::HashMap;
use std::{ path::Path, };

pub async fn run(Json(arg): Json<HashMap<String, String>>) -> impl IntoResponse {
    let code =arg.get("code").unwrap();
    let mut msg = vec![];
    async_std::task::block_on(async {
            let mut worker =  init(None).await;
            match  worker.js_runtime.execute_script("test", &code.clone()){
                Ok(data) => {
                    msg = get_msg().unwrap();
                },
                Err(e) => {
                    msg.push(e.to_string());
                },
            }
    });
    clear_msg();
    return RespVO::from(&msg.join("<br/>")).resp_json();
}

pub fn init_router() -> Router {
    Router::new().route("/js/run", post(run))
}
