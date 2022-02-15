use axum::{extract::extractor_middleware, response::IntoResponse, routing::get, Router};
use cassie_common::RespVO;
use cassie_web::{
    middleware::auth::Auth,
    routers::{admin, api},
    service::CONTEXT,
};


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
    // 初始化日志
    tracing_subscriber::fmt::init();
    //绑定端口 初始化 路由
    let app = Router::new()
        .route("/index", get(index))
        .nest("/admin", admin::routers().layer(extractor_middleware::<Auth>()))
        .nest("/api", api::routers());
    println!("address:{}", &CONTEXT.config.server);
    axum::Server::bind(&CONTEXT.config.server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
