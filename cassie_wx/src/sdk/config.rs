
use std::fs::File;
use cassie_common::error::Error;
use cassie_common::error::Result;
use cached::once_cell::sync::OnceCell;
static CONFIGS: OnceCell<Config> = OnceCell::new();
#[derive(Debug, Clone)]
pub enum PlatformType {
    OfficialAccount, // 公众号
    OpenPlatfrom,    // 开放平台
    MiniProgram,     // 小程序
}

/// 微信sdk配置
#[derive(Debug, Clone)]
pub struct Config {
    pub app_id: String,         // 应用id
    pub secret: String,         // 密钥
    pub token: String,          // token,在接口配置时填写的token,用于sigine验证
    pub platform: PlatformType, // 配置的平台类型
    // pub msg_type: MessageFormat,    // 消息格式
    // pub encrypt_mode: EncryptMode   // 加密方式
    pub mch_id: String,      //商户id
    pub private_key: String, //商户证书私钥
    pub certificate: String, //商户证书路径
    pub secret_key: String,  //API 秘钥
}
impl Config {
    pub fn load(params: serde_json::Value) -> Result<Config> {
        let _conf = Config {
            app_id: format!("{}", params["app_id"].as_str().unwrap_or_default()),
            secret: format!("{}", params["secret"].as_str().unwrap_or_default()),
            token: format!("{}", params["token"].as_str().unwrap_or_default()),
            platform: PlatformType::MiniProgram,
            mch_id: format!("{}", params["mch_id"].as_str().unwrap_or_default()),
            private_key: format!("{}", params["private_key"].as_str().unwrap_or_default()),
            certificate: format!("{}", params["certificate"].as_str().unwrap_or_default()),
            secret_key: format!("{}", params["secret_key"].as_str().unwrap_or_default()),
        };
        return Ok(_conf);
    }
        /// 获取对应参数
        pub fn get() -> Config {
            match CONFIGS.get() {
                Some(conf) => conf.clone(),
                None => Config::default(),
            }
        }
}
/// 默认配置项
impl Default for Config {
    fn default() -> Self {
        Config {
            app_id: String::new(),
            secret: String::new(),
            token: String::new(),
            platform: PlatformType::MiniProgram,
            mch_id: "".to_string(),
            private_key: "".to_string(),
            certificate: "".to_string(),
            secret_key: "".to_string()
        }
    }
}