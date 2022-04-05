#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]
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
pub mod interceptor;
pub mod observe;

use crate::cici_casbin::casbin_service::CasbinService;
use crate::interceptor::interceptor::AgencyInterceptor;
use crate::service::ServiceContext;
use cassie_config::config::ApplicationConfig;
use cassie_orm::dao::{init_mongdb, init_rbatis};
use mongodb::Database;
use observe::{consumer::init_consumer, event::CassieEvent};
use pharos::SharedPharos;
use rbatis::rbatis::Rbatis;
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
    //第二步初始化数据源
    init_database().await;
    //第三步初始化所有的 服务类
    APPLICATION_CONTEXT.set::<ServiceContext>(ServiceContext::new());
    println!("---------------------------------------ServiceContext配置完成--------------------------------------------");
    //第三步初始化casbinCContext
    APPLICATION_CONTEXT.set::<CasbinService>(CasbinService::default());
    println!("---------------------------------------CasbinService配置完成----------------------------------------------");
    init_event_bus().await;
    println!("---------------------------------------EventBus初始化完成------------------------------------------------");
}

//初始化 event bus事件处理器
pub async fn init_event_bus() {
    APPLICATION_CONTEXT.set::<SharedPharos<CassieEvent>>(SharedPharos::default());
    tokio::task::spawn(init_consumer());
}

pub async fn init_config() {
    println!("-------------------------------------正在启动--------------------------------------------------------");
    let yml_data = include_str!("../application.yml");
    let config = ApplicationConfig::new(yml_data);

    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
    println!("-------------------------------------yml配置完成-----------------------------------------------------");
}

pub async fn init_database() {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();

    let mut rbatis = init_rbatis(config).await;
    rbatis.add_sql_intercept(AgencyInterceptor {
        enable: config.tenant.enable.clone(),
        column: config.tenant.column.clone(),
        ignore_table: config.tenant.ignore_table.clone(),
    });
    APPLICATION_CONTEXT.set::<Rbatis>(rbatis);
    println!("---------------------------------------mysql配置完成------------------------------------------------------");
    let mdb = init_mongdb(config).await;
    APPLICATION_CONTEXT.set::<Database>(mdb);
    println!("---------------------------------------mongodb配置完成--------------------------------------------------");
}
