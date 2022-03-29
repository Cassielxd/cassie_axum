use super::upload_service::IUploadService;
use crate::CONTAINER;
use async_trait::async_trait;
use axum::body::Bytes;
use cassie_common::error::Error;
use cassie_common::error::Result;
use cassie_config::config::ApplicationConfig;
use oss_rust_sdk::prelude::*;
use std::collections::HashMap;
/**
 * @description:  IUploadService  upload base trait
 * @author String
 * @date 2022/3/25 15:54
 * @email 348040933@qq.com
 */
pub const CONTENT_TYPE: &str = "content-type";
pub struct OssService {
    access_endpoint: String,
}
impl OssService {
    pub fn new(access_endpoint: String) -> OssService {
        let config = CONTAINER.get::<ApplicationConfig>();
        CONTAINER.set::<OSS>(OSS::new(
            config.oss.key_id.as_str(),
            config.oss.key_secret.as_str(),
            config.oss.endpoint.as_str(),
            config.oss.bucket.as_str(),
        ));
        OssService {
            access_endpoint: access_endpoint,
        }
    }
}

#[async_trait]
impl IUploadService for OssService {
    async fn upload(&self, data: Bytes, file_name: String, content_type: String) -> Result<String> {
        let service = CONTAINER.get::<OSS>();
        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE, content_type.as_str());
        let result = service
            .async_put_object_from_buffer(&data, file_name.clone(), headers, None)
            .await;
        match result {
            Ok(_) => {
                let path = format!("{}/{}", self.access_endpoint.clone(), file_name.clone());
                return Ok(path);
            }
            Err(e) => Err(Error::E(e.to_string())),
        }
    }
}
