use axum::{response::IntoResponse, routing::get, Router};
use cassie_web::{
    routers::{admin, api},
    RespVO,
};
use std::net::SocketAddr;

pub async fn index() -> impl IntoResponse {
    RespVO::from(&"hello world".to_string()).resp_json()
}

/** 
 * axum 主入口类 
 * admin 后台管理端路由
 * api 前端使用路由 例如:pc wapp  app 
 * */
#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new()
        .route("/", get(index))
        .nest("/admin", admin::routers())
        .nest("/api", api::routers());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
