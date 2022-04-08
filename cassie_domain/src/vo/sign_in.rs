use crate::dto::sys_user_dto::SysUserDTO;
use serde::{Deserialize, Serialize};

/**
*struct:SignInVO
*desc:登录数据
*author:String
*email:348040933@qq.com
*/
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInVO {
    user: Option<SysUserDTO>,
    access_token: String,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
