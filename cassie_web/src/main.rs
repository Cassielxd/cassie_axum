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
        .route("/index", get(index))
        .layer(extractor_middleware::<Auth>())
        .nest("/admin", admin::routers())
        .nest("/api", api::routers());
    println!("address:{}", &CONTEXT.config.server);
    axum::Server::bind(&CONTEXT.config.server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
