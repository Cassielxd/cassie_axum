use serde::{Deserialize, Serialize};

/// 登陆
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct SignInDTO {
    #[validate(required)]
    #[validate(length(min = 5, message = "账号最少5个字符"))]
    pub username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 6, message = "密码最少6个字符"))]
    pub password: Option<String>,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    #[validate(required)]
    #[validate(length(equal = 4, message = "验证码必须为4位"))]
    pub vcode: Option<String>,
    #[validate(required)]
    pub uuid: Option<String>,
}

/// 验证码
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatpchaDTO {
    pub uuid: Option<String>,
}
