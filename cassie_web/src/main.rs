use std::time::Duration;

use axum::{
    extract::extractor_middleware,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use cassie_web::dao::mapper::get_menu_List_by_ids;
use cassie_web::{
    config::log::init_log,
    dao::mapper,
    middleware::auth::Auth,
    nacos::register_service,
    routers::{admin, api},
    CASSIE_CONFIG,
};
use log::info;
use tower_http::cors::{Any, CorsLayer};

pub async fn index() -> impl IntoResponse {
    Html(
        "<!DOCTYPE html>
    <html lang='zh-Hans'>
      <head>
        <meta charset='UTF-8' />
        <title>axum.rs</title>
      </head>
      <body>
        <p>欢迎使用 cassie axum</p>
        <p>这是一个学习性质的项目，又不懂的地方先看注释</p>
        <p>有疑问请加QQ:348040933</p>
      </body>
    </html>",
    )
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
        CASSIE_CONFIG.server.host.replace("0.0.0.0", "127.0.0.1"),
        CASSIE_CONFIG.server.port
    );
    //nacos 服务注册
    register_service().await;
    let server = format!(
        "{}:{}",
        CASSIE_CONFIG.server.host, CASSIE_CONFIG.server.port
    );
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(60) * 10);

    //绑定端口 初始化 路由
    let app = Router::new()
        .route("/", get(index))
        .nest(
            "/admin",
            admin::routers().layer(extractor_middleware::<Auth>()),
        )
        .nest("/api", api::routers())
        .layer(cors);

    Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
