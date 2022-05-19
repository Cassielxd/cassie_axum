use crate::initialize::rules::init;
use crate::service::build_script;
use crate::service::ops::{clear_msg, get_msg};
use crate::{get_local, CustomEvent};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use cassie_common::error::Error;
use cassie_common::RespVO;
use std::collections::HashMap;
use std::path::Path;

pub async fn run(Json(arg): Json<HashMap<String, String>>) -> impl IntoResponse {
    let request = get_local().unwrap();
    let cus = CustomEvent {
        path: request.path().clone(),
        params_values: None,
        return_values: None,
        request_model: Some(request),
    };
    let original_code = arg.get("code").unwrap();
    let mut msg = vec![];
    async_std::task::block_on(async {
        let mut worker = init(None).await;
        let init_code = format!(r#" var request_context={};"#, cus.as_json());
        let code = build_script(init_code.clone(), original_code.clone());
        match worker.js_runtime.execute_script("test.js", &code) {
            Ok(data) => {
                if let Some(m) = get_msg() {
                    msg = m;
                }
            }
            Err(e) => {
                msg.push(e.to_string());
            }
        }
    });
    clear_msg();
    return RespVO::from(&msg.join("<br/>")).resp_json();
}

pub fn init_router() -> Router {
    Router::new().route("/js/run", post(run))
}
