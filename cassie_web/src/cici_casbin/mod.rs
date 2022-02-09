pub mod casbin_service;

use casbin::rhai::ImmutableString;
use cassie_common::is_white_list_api;
use crate::cici_casbin::casbin_service::CasbinService;
use crate::service::CONTEXT;


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





lazy_static! {
    pub static ref CASBIN_CONTEXT: CasbinService = CasbinService::default();
}