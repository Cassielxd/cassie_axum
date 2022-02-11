mod cache_service;
mod mem_service;
mod redis_service;
mod sys_auth_service;
pub mod crud_service;
mod sys_user_service;
mod sys_role_service;
mod sys_role_menu_service;
mod sys_role_data_scope_service;
mod sys_role_user_service;
mod sys_menu_service;
mod sys_dict_service;
mod sys_params_service;

pub use sys_role_service::*;
pub use sys_user_service::*;
pub use sys_auth_service::*;
pub use sys_menu_service::*;
pub use sys_dict_service::*;
pub use sys_params_service::*;
use rbatis::rbatis::Rbatis;

pub use self::cache_service::*;
pub use self::mem_service::*;
pub use self::redis_service::*;
use crate::config::config::ApplicationConfig;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rbatis: Rbatis,
    pub cache_service: CacheService,
    /*权限服务 */
    pub  sys_auth_service: SysAuthService,
    /*用户服务 */
    pub sys_user_service: SysUserService,
    /*角色服务 */
    pub sys_role_service: SysRoleService,

    pub sys_menu_service: SysMenuService,
    pub sys_params_service:SysParamsService,
    /*数据字典服务 */
    pub sys_dict_type_service: SysDictTypeService,
    pub sys_dict_value_service: SysDictDataService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        Self {
            rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&config).await
            }),
            cache_service: CacheService::new(&config).unwrap(),
            sys_auth_service: Default::default(),
            sys_user_service: Default::default(),
            sys_role_service: Default::default(),
            sys_menu_service: Default::default(),
            sys_params_service: Default::default(),
            sys_dict_type_service: Default::default(),
            sys_dict_value_service: Default ::default(),
            config,

        }
    }
}

lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
