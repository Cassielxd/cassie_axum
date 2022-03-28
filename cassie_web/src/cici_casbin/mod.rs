pub mod casbin_service;

use casbin::rhai::ImmutableString;

use crate::{config::config::ApplicationConfig, CONTAINER};
use casbin::function_map::key_match2;

///是否处在白名单接口中
pub fn is_white_list_api(path: &str, white_list_api: &Vec<String>) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in white_list_api {
        //匹配 user/:id 模式
        if key_match2(path, x) || x.contains(path) {
            return true;
        }
    }
    return false;
}

pub fn is_super_admin(id: &str, super_admin_ids: &Vec<String>) -> bool {
    for x in super_admin_ids {
        if x == id {
            return true;
        }
    }
    return false;
}

/**
 *method:cici_match
 *desc:验证白名单 验证是不是管理员
 *author:String
 *email:348040933
 */
pub fn cici_match(user: ImmutableString, path: ImmutableString) -> bool {
    let cassie_config = CONTAINER.get::<ApplicationConfig>();
    if is_white_list_api(&path, &cassie_config.admin_white_list_api) {
        println!("白名单:{}", path.clone());
        return true;
    }

    if !user.is_empty() && is_super_admin(&user, &cassie_config.super_admin_ids) {
        println!("管理员");
        return true;
    }
    return false;
}
