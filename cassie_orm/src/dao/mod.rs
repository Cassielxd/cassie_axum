use cassie_config::config::ApplicationConfig;
use rbatis::rbatis::Rbatis;
pub mod interceptor;
pub mod mapper;
use interceptor::*;

///实例化 rbatis orm 连接池
pub async fn init_rbatis(cassie_config: &ApplicationConfig) -> Rbatis {
    let mut rbatis = Rbatis::new();
    if cassie_config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    rbatis.add_sql_intercept(AgencyInterceptor {});
    //连接数据库
    println!(
        "rbatis link database ({})...",
        cassie_config.database_url.clone()
    );
    rbatis
        .link(&cassie_config.database_url)
        .await
        .expect("rbatis link database fail!");
    println!("rbatis link database success!");

    return rbatis;
}

use mongodb::{options::ClientOptions, Client, Database};

pub async fn init_mongdb(cassie_config: &ApplicationConfig) -> Database {
    let client_options = ClientOptions::parse(cassie_config.mongodb_url.clone().as_str())
        .await
        .expect(" mongdb link database fail!");
    println!(
        "mongdb link database ({})...",
        cassie_config.mongodb_url.clone()
    );
    let client = Client::with_options(client_options).unwrap();
    println!("mongdb link database success!");
    let db = client.database("cassie");
    db
}
