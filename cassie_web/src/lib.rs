#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cached;
//配置
pub mod config;
//管理端
pub mod admin;
//权限模块
pub mod cici_casbin;
//中间件
pub mod middleware;
//nacos注册中心默认实现
pub mod nacos;
//路由
pub mod routers;
//总服务入口
pub mod service;
//前端接口
pub mod api;
pub mod initialize;
pub mod interceptor;
pub mod observe;
pub mod ws;

use std::collections::HashMap;

use crate::initialize::casbin::init_casbin;
use crate::initialize::config::init_config;
use crate::initialize::database::init_database;
use crate::initialize::event::init_event_bus;
use crate::initialize::rules::init_rules;
use crate::initialize::service::init_service;
use crate::interceptor::interceptor::AgencyInterceptor;
use crate::nacos::register_service;
use crate::ws::ws_server::init_ws;
use crate::{cici_casbin::casbin_service::CasbinService, config::log::init_log};
use cassie_config::config::ApplicationConfig;
pub use deno_runtime::deno_core;
use log::info;
use middleware::get_local;
use observe::event::CustomEvent;
use observe::{consumer::init_consumer, event::CassieEvent};
use service::fire_event;
use state::Container;
/*
整个项目上下文ApplicationContext
包括：
ApplicationConfig 配置
Database mongodb数据库
Rbatis  mysql orm
ServiceContext 服务上下文
CasbinService 权限服务
*/

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();
/*初始化环境上下文*/
pub async fn init_context() {
    //第一步加载配置
    init_config().await;
    init_log();
    info!("ConfigContext init complete");
    //第二步初始化数据源
    init_database().await;
    info!("DataBase init complete");
    //第三步初始化所有的 服务类
    init_service().await;
    info!("ServiceContext init complete");
    //第三步初始化casbinCContext
    init_casbin().await;
    info!("CasbinService init complete");
    init_rules().await;
    info!("RulesContext init complete");
    init_event_bus().await;
    info!("EventBus init complete");
    tokio::spawn(init_ws());
    //nacos 服务注册
    register_service().await;
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    info!(" - Local:   http://{}:{}", cassie_config.server().host().replace("0.0.0.0", "127.0.0.1"), cassie_config.server().port());
}

async fn fire_script_event(params_values: HashMap<String, serde_json::Value>, return_values: serde_json::Value) {
    let request = get_local();
    match request {
        Some(data) => {
            let cus = CustomEvent {
                path: data.path().clone(),
                params_values: Some(params_values),
                return_values,
                request_model: Some(data),
            };
            let event = CassieEvent::Custom(cus);
            fire_event(event).await;
        }
        None => {}
    }
}
