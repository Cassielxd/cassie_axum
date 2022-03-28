#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NacosConfig {
    pub nacos_flag: bool,
    pub nacos_server: String,
    pub application_name: String,
}
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    ///当前服务地址
    pub host: String,
    pub port: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TenantConfig {
    pub enable: bool,
    pub column: String,
    pub ignore_table: Vec<String>,
}
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OSSConfig {
    pub key_id: String,
    pub key_secret: String,
    pub endpoint: String,
    pub bucket: String,
    pub access_endpoint: String,
}
impl OSSConfig {
    pub fn validate(&self) {
        if self.key_id.is_empty()
            || self.key_secret.is_empty()
            || self.endpoint.is_empty()
            || self.bucket.is_empty()
        {
            panic!("请配置oss ！！！！！！！！！！！！！！！！！！！")
        }
    }
}

///服务启动配置
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    ///redis地址
    pub redis_url: String,
    //mongodb_url地址
    pub mongodb_url: String,
    /// 数据库地址
    pub database_url: String,
    /// 逻辑删除字段
    pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64,
    ///日志目录 "target/logs/"
    pub log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    pub log_pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    ///日志等级
    pub log_level: String,
    ///短信缓存队列（mem/redis）
    pub sms_cache_send_key_prefix: String,
    ///jwt 秘钥
    pub jwt_secret: String,
    ///白名单接口
    pub admin_white_list_api: Vec<String>,
    pub api_white_list_api: Vec<String>,
    pub super_admin_ids: Vec<String>,
    ///权限缓存类型
    pub cache_type: String,
    pub upload_type: String,
    ///重试
    pub login_fail_retry: u64,
    ///重试等待时间
    pub login_fail_retry_wait_sec: u64,
    //server 配置
    pub server: ServerConfig,
    //nacos 配置
    pub nacos: NacosConfig,
    //租户 配置
    pub tenant: TenantConfig,
    //oss 配置
    pub oss: OSSConfig,
}

///默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {
        println!("读取 application.yml");
        let yml_data = include_str!("../../application.yml");
        //读取配置
        let result: ApplicationConfig = serde_yaml::from_str(yml_data).expect("配置文件加载失败");
        println!("配置 ApplicationConfig");
        result
    }
}
