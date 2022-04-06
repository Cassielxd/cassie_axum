use axum::{
    extract::extractor_middleware,
    handler::Handler,
    http::Uri,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use cassie_common::RespVO;
use cassie_config::config::ApplicationConfig;
use cassie_web::{
    config::log::init_log,
    init_context,
    middleware::{auth::Auth, event::EventMiddleware},
    nacos::register_service,
    routers::{admin, api},
    APPLICATION_CONTEXT,
};
use log::info;
use reqwest::StatusCode;
use std::time::Duration;
use tower::layer::layer_fn;
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
async fn fallback(uri: Uri) -> impl IntoResponse {
    let msg = format!("资源不存在：{}", uri);
    RespVO::<String> {
        code: Some(-1),
        msg: Some(msg),
        data: None,
    }
    .resp_json()
}
/**
 *method:main
 *desc:程序主入口方法 admin 管理端api api:小程序,h5,app使用
 *author:String
 *email:348040933QQ.com
 */
#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    init_log();
    info!(
        " - Local:   http://{}:{}",
        cassie_config.server.host.replace("0.0.0.0", "127.0.0.1"),
        cassie_config.server.port
    );
    //nacos 服务注册
    register_service().await;
    let server = format!(
        "{}:{}",
        cassie_config.server.host, cassie_config.server.port
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
            admin::routers()
                .layer(layer_fn(|inner| EventMiddleware { inner })) //第二执行的
                .layer(extractor_middleware::<Auth>()), //最先执行的
        )
        .nest("/api", api::routers())
        .fallback(fallback.into_service())
        .layer(cors);
    // 启动服务
    Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
