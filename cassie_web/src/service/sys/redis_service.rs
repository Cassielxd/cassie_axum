use async_trait::async_trait;
use cassie_common::error::{Error, Result};

use super::cache_service::ICassieCacheService;
use cached::{AsyncRedisCache, IOCachedAsync};
///Redis缓存服务
pub struct CassieRedisService {
    pub client: AsyncRedisCache<String, String>,
}
impl CassieRedisService {
    pub async fn new(url: &str) -> Self {
        println!("conncect redis ({})...", url);
        let client = AsyncRedisCache::<String, String>::new("_cassie_".to_string(), 360)
            .set_connection_string(url)
            .set_refresh(true)
            .build()
            .await
            .unwrap();
        println!("conncect redis success!");
        Self { client }
    }
}
#[async_trait]
impl ICassieCacheService for CassieRedisService {
    async fn cache_set(&self, k: &str, v: &str) -> Result<String> {
        let res = self.client.cache_set(k.to_string(), v.to_string()).await;

        match res {
            Ok(data) => Ok(data.unwrap_or_default()),
            Err(e) => Err(Error::E(format!(
                "CassieRedisService cache_remove fail:{}",
                e.to_string()
            ))),
        }
    }

    async fn cache_get(&self, k: &str) -> Result<String> {
        let res = self.client.cache_get(&k.to_string()).await;
        match res {
            Ok(data) => Ok(data.unwrap_or_default()),
            Err(e) => Err(Error::E(format!(
                "CassieRedisService cache_get fail:{}",
                e.to_string()
            ))),
        }
    }

    async fn cache_remove(&self, k: &str) -> Result<String> {
        let res = self.client.cache_remove(&k.to_string()).await;
        match res {
            Ok(data) => Ok(data.unwrap_or_default()),
            Err(e) => Err(Error::E(format!(
                "CassieRedisService cache_remove fail:{}",
                e.to_string()
            ))),
        }
    }
}
