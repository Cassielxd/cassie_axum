use axum::extract::Multipart;
use axum::routing::post;
use axum::Router;
use cassie_common::RespVO;

use crate::APPLICATION_CONTEXT;
use axum::response::IntoResponse;
use cassie_common::error::Error;
use cassie_upload::upload::upload_service::UploadService;

pub const CONTENT_TYPE: &str = "content-type";

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    let upload_service = APPLICATION_CONTEXT.get::<UploadService>();
    if let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let result = upload_service.upload(data, file_name, content_type).await;
        match result {
            Ok(s) => {
                return RespVO::from(&s).resp_json();
            }
            Err(e) => {
                return RespVO::<()>::from_error(&Error::from(e.to_string())).resp_json();
            }
        }
    }
    return RespVO::<()>::from_error(&Error::from("params error".to_string())).resp_json();
}

pub fn init_router() -> Router {
    Router::new().route("/upload", post(upload))
}
