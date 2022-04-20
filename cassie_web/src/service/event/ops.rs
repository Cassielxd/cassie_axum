use cassie_config::config::ApplicationConfig;
use cassie_domain::dto::{sys_dict_dto::SysDictTypeDTO, sys_user_dto::SysUserDTO};
use deno_core::{op, Extension};

use crate::{
    service::{
        crud_service::CrudService, sys_dict_service::get_all_list, sys_user_service::SysUserService,
    },
    APPLICATION_CONTEXT,
};

//获取系统配置
#[op]
fn op_config() -> Result<ApplicationConfig, deno_core::error::AnyError> {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    Ok(config.clone())
}
//获取系统字典
#[op]
fn op_all_dict() -> Result<Vec<SysDictTypeDTO>, deno_core::error::AnyError> {
    let vo = async_std::task::block_on(async { get_all_list().await });
    Ok(vo.unwrap())
}

//获取用户信息
#[op]
fn op_user_info(id: String) -> Result<SysUserDTO, deno_core::error::AnyError> {
    let user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    let vo = async_std::task::block_on(async { user_service.get(id).await });
    Ok(vo.unwrap())
}

pub fn init_sys_ops() -> Extension {
    Extension::builder()
        .ops(vec![
            op_user_info::decl(),
            op_all_dict::decl(),
            op_config::decl(),
        ])
        .build()
}
