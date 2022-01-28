pub mod cache_service;
pub mod mem_service;
pub mod redis_service;

use rbatis::rbatis::Rbatis;

pub use self::cache_service::*;
pub use self::mem_service::*;
pub use self::redis_service::*;
use crate::config::config::ApplicationConfig;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        Self {
            rbatis:async_std::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            cache_service: CacheService::new(&config).unwrap(),
            config,
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
