use crate::service::redis_service::CassieRedisService;
use crate::APPLICATION_CONTEXT;
use async_trait::async_trait;
use cassie_common::error::Result;
use cassie_config::config::ApplicationConfig;
use serde::de::DeserializeOwned;
use serde::Serialize;
//缓存服务接口
pub struct CacheService {
    pub inner: Box<dyn ICassieCacheService>,
}

impl CacheService {
    pub fn new() -> Self {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        match cassie_config.cache_type.as_str() {
            "redis" => {
                println!("cache_type: redis");
                Self {
                    inner: Box::new(async_std::task::block_on(async {
                        CassieRedisService::new(&cassie_config.redis_url).await
                    })),
                }
            }
            e => {
                panic!(
                    "unknown of cache_type: \"{}\",current support 'mem' or 'redis'",
                    e
                );
            }
        }
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.inner.cache_set(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        self.inner.cache_get(k).await
    }
    pub async fn remove_string(&self, k: &str) -> Result<String> {
        self.inner.cache_remove(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(cassie_common::error::Error::from(format!(
                "MemCacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string(k, data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(cassie_common::error::Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }
}

#[async_trait]
pub trait ICassieCacheService: Sync + Send {
    async fn cache_set(&self, k: &str, v: &str) -> Result<String>;

    async fn cache_get(&self, k: &str) -> Result<String>;

    async fn cache_remove(&self, k: &str) -> Result<String>;
}
