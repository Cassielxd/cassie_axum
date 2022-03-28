use rbatis::rbatis::Rbatis;

pub mod interceptor;
pub mod mapper;
use interceptor::*;

///实例化 rbatis orm 连接池
pub async fn init_rbatis() -> Rbatis {
    let mut rbatis = Rbatis::new();
    let CASSIE_CONFIG= CONTAINER.get::<ApplicationConfig>();
    if CASSIE_CONFIG.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    rbatis.add_sql_intercept(AgencyInterceptor {});
    //连接数据库
    println!(
        "rbatis link database ({})...",
        CASSIE_CONFIG.database_url.clone()
    );
    rbatis
        .link(&CASSIE_CONFIG.database_url)
        .await
        .expect("rbatis link database fail!");
    println!("rbatis link database success!");

    return rbatis;
}

use mongodb::{
    options::ClientOptions,
    Client, Database,
};

use crate::{CONTAINER, config::config::ApplicationConfig};
pub async fn init_mongdb() -> Database {
    let CASSIE_CONFIG= CONTAINER.get::<ApplicationConfig>();
    let  client_options = ClientOptions::parse(CASSIE_CONFIG.mongodb_url.clone().as_str())
        .await
        .expect(" mongdb link database fail!");
        
    let client = Client::with_options(client_options).unwrap();
    println!("mongdb link database success!");
    let db = client.database("cassie");
  
    db
}
