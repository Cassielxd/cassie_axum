
use casbin::rhai::ImmutableString;
use cassie_common::is_white_list_api;

use crate::{service::CONTEXT, cici_casbin::casbin_service::CasbinService};

pub mod action;
pub mod models;
pub mod cici_adapter;

/**
*method:cici_match
*desc:验证白名单 验证是不是管理员
*author:String
*email:348040933
*/
pub fn cici_match(sub: ImmutableString, obj: ImmutableString) -> bool {
    if is_white_list_api(&obj,&CONTEXT.config.white_list_api) {
       println!("白名单:{}",obj.clone());
        return true;
    }
    if !sub.is_empty() && sub == "admin" {
        println!("管理员");
        return true;
    }
    return false;
}




pub struct CasbinContext {
    pub service: CasbinService,
}



impl CasbinContext {
    fn default() -> CasbinContext {
        Self{
            service:async_std::task::block_on(async { CasbinService::default().await })
        }
    }
    
}

lazy_static! {
    pub static ref CASBIN_CONTEXT: CasbinContext = CasbinContext::default();
}