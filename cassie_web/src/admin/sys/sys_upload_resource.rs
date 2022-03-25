use axum::extract::Multipart;
use axum::routing::post;
use axum::Router;
use oss_rust_sdk::prelude::*;
use std::collections::HashMap;

pub const CONTENT_TYPE: &str = "content-type";

async fn upload(mut multipart: Multipart) {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let content_type = field.content_type().unwrap().to_string();
        let oss_instance = OSS::new(
            "your_AccessKeyId",
            "your_AccessKeySecret",
            "your_Endpoint",
            "your_Bucket",
        );
        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE, content_type.as_str());
        let result = oss_instance
            .async_put_object_from_buffer(&data, name, headers, None)
            .await;
    }
}

pub fn init_router() -> Router {
    Router::new().route("/upload", post(upload))
}
