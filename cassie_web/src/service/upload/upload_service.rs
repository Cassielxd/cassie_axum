use crate::{service::upload::oss_service::OssService, CONTAINER};
use async_trait::async_trait;
use axum::body::Bytes;
use cassie_common::error::Result;
use cassie_config::config::ApplicationConfig;
/**
 * @description:  IUploadService  upload base trait
 * @author String
 * @date 2022/3/25 15:54
 * @email 348040933@qq.com
 */
#[async_trait]
pub trait IUploadService: Sync + Send {
    async fn upload(&self, data: Bytes, file_name: String, content_type: String) -> Result<String>;
}

pub struct UploadService {
    pub inner: Box<dyn IUploadService>,
}

impl UploadService {
    pub fn new() -> cassie_common::error::Result<Self> {
        let config = CONTAINER.get::<ApplicationConfig>();
        match config.upload_type.as_str() {
            "oss" => {
                println!("---------------------------------存储类型oss-----------------------------------------------------");
                config.oss.validate();
                Ok(Self {
                    inner: Box::new(OssService::new(config.oss.access_endpoint.clone())),
                })
            }
            e => {
                panic!("unknown of upload_type: \"{}\",current support 'oss' ", e);
            }
        }
    }
    pub async fn upload(
        &self,
        data: Bytes,
        file_name: String,
        content_type: String,
    ) -> Result<String> {
        self.inner.upload(data, file_name, content_type).await
    }
}
