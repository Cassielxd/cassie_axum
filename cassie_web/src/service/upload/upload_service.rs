use async_trait::async_trait;
use axum::body::Bytes;
use cassie_common::error::Result;
use rbatis::bytes;

use crate::{CASSIE_CONFIG, service::upload::oss_service::OssService};
/**
 * @description:  IUploadService  upload base trait
 * @author String
 * @date 2022/3/25 15:54
 * @email 348040933@qq.com
 */
#[async_trait]
pub trait IUploadService: Sync + Send {
    async fn upload(&self,data:Bytes,file_name:String,content_type:String)->Result<String>;
}

pub struct UploadService {
    pub inner: Box<dyn IUploadService>,
}

impl UploadService {
    pub fn new() -> cassie_common::error::Result<Self> {
        match CASSIE_CONFIG.upload_type.as_str() {
            "oss" => {
                println!(" upload_type: oss");
                Ok(Self {
                    inner: Box::new(OssService::new(CASSIE_CONFIG.oss.access_endpoint.clone())),
                })
            }
            e => {
                panic!(
                    "unknown of cache_type: \"{}\",current support 'mem' or 'redis'",
                    e
                );
            }
        }
    }
    pub async fn upload(&self,data:Bytes,file_name:String,content_type:String)->Result<String>{
        self.inner.upload(data, file_name, content_type).await
    }
}