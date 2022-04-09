use serde::{Deserialize, Serialize};

/// 登陆
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SignInDTO {
    #[validate(required)]
    #[validate(length(min = 5, message = "账号最少5个字符"))]
    username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 6, message = "密码最少6个字符"))]
    password: Option<String>,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    vcode: Option<String>,
    #[validate(required)]
    uuid: Option<String>,
}

/// 验证码
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CatpchaDTO {
    pub uuid: Option<String>,
}
