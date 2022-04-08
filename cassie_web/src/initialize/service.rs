use crate::service::asi::asi_service::AsiGroupService;
use crate::service::cache_service::CacheService;
use crate::service::log::log_service::{LogLoginService, LogOperationService};
use crate::service::sys_auth_service::SysAuthService;
use crate::service::sys_dict_service::{SysDictDataService, SysDictTypeService};
use crate::service::sys_menu_service::SysMenuService;
use crate::service::sys_params_service::SysParamsService;
use crate::service::sys_role_service::SysRoleService;
use crate::service::sys_user_service::SysUserService;
use crate::APPLICATION_CONTEXT;
use cassie_config::config::ApplicationConfig;
use cassie_upload::upload::upload_service::UploadService;

pub async fn init_service() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    APPLICATION_CONTEXT.set::<CacheService>(CacheService::new());
    APPLICATION_CONTEXT.set::<SysAuthService>(SysAuthService::default());
    APPLICATION_CONTEXT.set::<SysUserService>(SysUserService::default());
    APPLICATION_CONTEXT.set::<SysRoleService>(SysRoleService::default());
    APPLICATION_CONTEXT.set::<SysMenuService>(SysMenuService::default());
    APPLICATION_CONTEXT.set::<SysParamsService>(SysParamsService::default());
    APPLICATION_CONTEXT.set::<SysDictTypeService>(SysDictTypeService::default());
    APPLICATION_CONTEXT.set::<SysDictDataService>(SysDictDataService::default());
    APPLICATION_CONTEXT.set::<AsiGroupService>(AsiGroupService::default());
    APPLICATION_CONTEXT.set::<UploadService>(UploadService::new(config).unwrap());
    APPLICATION_CONTEXT.set::<LogLoginService>(LogLoginService::default());
    APPLICATION_CONTEXT.set::<LogOperationService>(LogOperationService::default());
}
