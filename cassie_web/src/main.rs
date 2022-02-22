use axum::{extract::extractor_middleware, response::IntoResponse, routing::get, Router};
use cassie_common::RespVO;
use cassie_web::{
    CASSIE_CONFIG,
    config::{log::init_log},
    middleware::auth::Auth,
    routers::{admin, api}, nacos::{ping_schedule, register_service}
};
use log::info;

pub async fn index() -> impl IntoResponse {
    RespVO::from(&"hello world".to_string()).resp_json()
}

/**
 *method:main
 *desc:程序主入口方法 admin 管理端api api:小程序,h5,app使用
 *author:String
 *email:348040933QQ.com
 */
#[tokio::main]
async fn main() {
    init_log();
    info!(
        " - Local:   http://{}:{}",
        CASSIE_CONFIG.host.replace("0.0.0.0", "127.0.0.1"),CASSIE_CONFIG.port
    );
    register_service();
   let server= format!("{}:{}",CASSIE_CONFIG.host,CASSIE_CONFIG.port);
    //绑定端口 初始化 路由
    let app = Router::new()
        .route("/index", get(index))
        .nest(
            "/admin",
            admin::routers().layer(extractor_middleware::<Auth>()),
        )
        .nest("/api", api::routers());
        
    axum::Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
        ping_schedule();
        
}
