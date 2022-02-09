pub mod cache_service;
pub mod mem_service;
pub mod redis_service;
pub mod sys_auth_service;

use rbatis::rbatis::Rbatis;

pub use self::cache_service::*;
pub use self::mem_service::*;
pub use self::redis_service::*;
use crate::config::config::ApplicationConfig;
use crate::service::sys_auth_service::SysAuthService;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
    /*权限服务 */
    pub sys_auth_service: SysAuthService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        Self {
            rbatis:async_std::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            cache_service: CacheService::new(&config).unwrap(),
            sys_auth_service:SysAuthService::default(),
            config,
        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
