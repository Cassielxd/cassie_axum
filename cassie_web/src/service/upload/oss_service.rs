use async_trait::async_trait;
use axum::body::Bytes;
use oss_rust_sdk::prelude::*;
use cassie_common::error::Result;
use crate::CASSIE_CONFIG;

use super::upload_service::IUploadService;
use std::collections::HashMap;
use cassie_common::error::Error;
/**
 * @description:  IUploadService  upload base trait
 * @author String
 * @date 2022/3/25 15:54
 * @email 348040933@qq.com
 */
pub const CONTENT_TYPE: &str = "content-type";
pub struct OssService {
    access_endpoint:String
}
impl OssService {
    pub fn new(access_endpoint:String)->OssService {
        OssService{
            access_endpoint:access_endpoint
        }
    }
}

#[async_trait]
impl IUploadService for OssService {
    async fn upload(&self,data:Bytes,file_name:String,content_type:String)->Result<String>{
        let mut headers = HashMap::new();
        headers.insert(CONTENT_TYPE, content_type.as_str());
        let result = oss_client.async_put_object_from_buffer(&data, file_name.clone(), headers, None)
        .await;
        match result {
            Ok(_) => {
                let path = format!("{}/{}", self.access_endpoint.clone(), file_name.clone());
                return Ok(path);
            }
            Err(e) => {
                Err(Error::E(e.to_string()))
            }
        }
    }

}
async fn init_oss()->OSS<'static>{
    let oss_instance = OSS::new(
        CASSIE_CONFIG.oss.key_id.as_str(),
        CASSIE_CONFIG.oss.key_secret.as_str(),
        CASSIE_CONFIG.oss.endpoint.as_str(),
        CASSIE_CONFIG.oss.bucket.as_str(),
    );
    oss_instance
}
lazy_static! {

    pub static ref oss_client:OSS<'static>= async_std::task::block_on(async { init_oss().await });

}