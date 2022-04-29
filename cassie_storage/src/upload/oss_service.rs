use super::upload_service::IUploadService;
use super::OSS_CLIENT;
use async_trait::async_trait;
use axum::body::Bytes;
use cassie_common::error::Error;
use cassie_common::error::Result;
use oss_rust_sdk::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

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
  pub fn new(key_id: String, key_secret: String, endpoint: String, bucket: String, access_endpoint: String) -> OssService {
    let client = Arc::new(OSS::new(key_id, key_secret, endpoint, bucket));
    OSS_CLIENT.set(client);
    OssService {
      access_endpoint: access_endpoint.clone(),
    }
  }
}

#[async_trait]
impl IUploadService for OssService {
  async fn upload(&self, data: Bytes, file_name: String, content_type: String) -> Result<String> {
    let mut headers = HashMap::new();
    headers.insert(CONTENT_TYPE, content_type.as_str());
    let client: &OSS = OSS_CLIENT.get();
    let result = client.async_put_object_from_buffer(&data, file_name.clone(), headers, None).await;
    match result {
      Ok(_) => {
        let path = format!("{}/{}", self.access_endpoint.clone(), file_name.clone());
        return Ok(path);
      }
      Err(e) => Err(Error::E(e.to_string())),
    }
  }
}
