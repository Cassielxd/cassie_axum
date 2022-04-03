use crate::service::redis_service::CassieRedisService;
use crate::APPLICATION_CONTEXT;
use async_trait::async_trait;
use cassie_common::error::Result;
use cassie_config::config::ApplicationConfig;
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
}

#[async_trait]
pub trait ICassieCacheService: Sync + Send {
    async fn cache_set(&self, k: &str, v: &str) -> Result<String>;

    async fn cache_get(&self, k: &str) -> Result<String>;

    async fn cache_remove(&self, k: &str) -> Result<String>;
}
