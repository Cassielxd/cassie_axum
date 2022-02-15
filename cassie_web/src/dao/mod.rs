use crate::config::config::ApplicationConfig;
/* use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin; */
use rbatis::rbatis::Rbatis;

pub mod mapper;
pub mod interceptor;
use interceptor::*;

///实例化 rbatis orm 连接池
pub async fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let mut rbatis = Rbatis::new();

    if config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    rbatis.add_sql_intercept(AgencyInterceptor{});
    //连接数据库
    println!("rbatis link database ({})...", config.database_url);
    rbatis
        .link(&config.database_url)
        .await
        .expect("rbatis link database fail!");
    println!("rbatis link database success!");
    return rbatis;
}
