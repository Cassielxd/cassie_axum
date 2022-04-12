pub mod casbin_service;

use crate::APPLICATION_CONTEXT;
use cached::proc_macro::cached;
use casbin::function_map::key_match2;
use casbin::rhai::ImmutableString;
use cassie_config::config::ApplicationConfig;

//需要登录但是不需要权限的路由比如获取用户 修改密码等
pub fn match_base_url(url: &str) -> bool {
    false
}

///是否处在白名单接口中
#[cached(name = "admin_white_list_api", time = 60, size = 100)]
pub fn is_white_list_api(path: String) -> bool {
    if path.eq("/") {
        return true;
    }
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    for x in cassie_config.admin_white_list_api() {
        //匹配 user/:id 模式
        if key_match2(path.clone().as_str(), x) || x.contains(path.clone().as_str()) {
            return true;
        }
    }
    return false;
}
#[cached(name = "admin_auth_list_api", time = 60, size = 100)]
pub fn is_auth_list_api(path: String) -> bool {
    if path.eq("/") {
        return true;
    }
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    for x in cassie_config.admin_auth_list_api().clone().unwrap() {
        //匹配 user/:id 模式
        if key_match2(path.clone().as_str(), &x) || x.contains(path.clone().as_str()) {
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
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    if is_white_list_api(path.clone().to_string()) {
        return true;
    }

    if !user.is_empty() && is_super_admin(&user, cassie_config.super_admin_ids()) {
        return true;
    }
    return false;
}
