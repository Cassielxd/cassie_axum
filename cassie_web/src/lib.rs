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
pub mod initialize;
pub mod interceptor;
pub mod observe;

use crate::cici_casbin::casbin_service::CasbinService;
use crate::initialize::casbin::init_casbin;
use crate::initialize::config::init_config;
use crate::initialize::database::init_database;
use crate::initialize::event::init_event_bus;
use crate::initialize::rules::init_rules;
use crate::initialize::service::init_service;
use crate::interceptor::interceptor::AgencyInterceptor;
use observe::{consumer::init_consumer, event::CassieEvent};
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
    println!("-------------------------------------CassieApplication正在启动--------------------------------------------------------");
    //第一步加载配置
    init_config().await;
    println!("-------------------------------------ConfigContext配置完成-----------------------------------------------------");
    //第二步初始化数据源
    init_database().await;
    println!("---------------------------------------DataBase配置完成------------------------------------------------------");
    //第三步初始化所有的 服务类
    init_service().await;
    println!("---------------------------------------ServiceContext配置完成--------------------------------------------");
    //第三步初始化casbinCContext
    init_casbin().await;
    println!("---------------------------------------CasbinService配置完成----------------------------------------------");
    init_rules().await;
    println!("---------------------------------------RulesContext配置完成----------------------------------------------");
    init_event_bus().await;
    println!("---------------------------------------EventBus初始化完成------------------------------------------------");
}
