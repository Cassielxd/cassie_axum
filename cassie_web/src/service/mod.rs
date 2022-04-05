pub mod asi;
pub mod event;
pub mod sys;

use crate::APPLICATION_CONTEXT;

use self::{
    asi::asi_service::AsiGroupService,
    cache_service::CacheService,
    sys_auth_service::SysAuthService,
    sys_dict_service::{SysDictDataService, SysDictTypeService},
    sys_menu_service::SysMenuService,
    sys_params_service::SysParamsService,
    sys_role_service::SysRoleService,
    sys_user_service::SysUserService,
};
use cassie_config::config::ApplicationConfig;
use cassie_upload::upload::upload_service::UploadService;
pub use event::*;
pub use sys::*;
pub struct ServiceContext {
    pub cache_service: CacheService,
    /*权限服务 */
    pub sys_auth_service: SysAuthService,
    /*用户服务 */
    pub sys_user_service: SysUserService,
    /*角色服务 */
    pub sys_role_service: SysRoleService,

    pub sys_menu_service: SysMenuService,
    pub sys_params_service: SysParamsService,
    /*数据字典服务 */
    pub sys_dict_type_service: SysDictTypeService,
    pub sys_dict_value_service: SysDictDataService,
    /*动态表单服务 */
    pub asi_service: AsiGroupService,
    /*上传服务*/
    pub upload_service: UploadService,
}

impl ServiceContext {
    pub fn new() -> Self {
        let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        Self {
            cache_service: CacheService::new(),
            sys_auth_service: Default::default(),
            sys_user_service: Default::default(),
            sys_role_service: Default::default(),
            sys_menu_service: Default::default(),
            sys_params_service: Default::default(),
            sys_dict_type_service: Default::default(),
            sys_dict_value_service: Default::default(),
            asi_service: Default::default(),
            upload_service: UploadService::new(config).unwrap(),
        }
    }
}
