use std::sync::{Arc, Mutex};

use cassie_config::config::ApplicationConfig;
use cassie_domain::dto::{sys_dict_dto::SysDictTypeDTO, sys_user_dto::SysUserDTO};
use deno_core::{op, Extension, OpDecl};
use log::info;

use crate::{
    service::{crud_service::CrudService, sys_dict_service::get_all_list, sys_user_service::SysUserService},
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
pub fn op_user_info(id: String) -> Result<SysUserDTO, deno_core::error::AnyError> {
    let user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    let vo = async_std::task::block_on(async { user_service.get(id).await });
    Ok(vo.unwrap())
}

pub fn init_sys_ops() -> Extension {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let mut builder = Extension::builder();

    //如果是开发环境 则重写 op_print 方法
    let f = |op: OpDecl| match op.name {
        "op_print" => op_print::decl(),
        _ => op,
    };
    if *config.debug() {
        builder.middleware(f);
    }
    builder.ops(vec![op_user_info::decl(), op_all_dict::decl(), op_config::decl(), op_print::decl()]).build()
}

#[op]
pub fn op_print(msg: String, is_err: bool) -> Result<(), deno_core::error::AnyError> {
    set_msg(msg);
    Ok(())
}

pub fn get_msg() -> Option<Vec<String>> {
    let request_model = APPLICATION_CONTEXT.try_get_local::<Arc<Mutex<Vec<String>>>>();
    match request_model {
        None => None,
        Some(e) => {
            let e = e.lock().unwrap();
            Some(e.clone())
        }
    }
}

pub fn set_msg(msg: String) {
    let request_model = APPLICATION_CONTEXT.try_get_local::<Arc<Mutex<Vec<String>>>>();
    match request_model {
        Some(d) => {
            let a = d.clone();
            let mut vec = a.lock().unwrap();
            vec.push(msg);
        }
        None => {
            APPLICATION_CONTEXT.set_local(move || {
                let mut vec = Vec::<String>::new();
                vec.push(msg.clone());
                let mutex = Arc::new(Mutex::new(vec));
                return mutex;
            });
        }
    }
}
pub fn clear_msg() {
    let request_model = APPLICATION_CONTEXT.try_get_local::<Arc<Mutex<Vec<String>>>>();
    match request_model {
        Some(d) => {
            let a = d.clone();
            let mut vec = a.lock().unwrap();
            *vec = Vec::<String>::new();
        }
        None => {
            APPLICATION_CONTEXT.set_local(move || {
                let vec = Vec::<String>::new();
                let mutex = Arc::new(Mutex::new(vec));
                return mutex;
            });
        }
    }
}
