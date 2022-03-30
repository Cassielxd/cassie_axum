#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]
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

use cassie_orm::dao::{init_mongdb, init_rbatis};
use mongodb::Database;
use rbatis::rbatis::Rbatis;
use crate::cici_casbin::casbin_service::CasbinService;
use crate::service::ServiceContext;
use cassie_config::config::ApplicationConfig;
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
    println!("-------------------------------------正在启动--------------------------------------------------------");
    let yml_data = include_str!("../application.yml");
    let config = ApplicationConfig::new(yml_data);
    config.validate();
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    println!("-------------------------------------yml配置完成-----------------------------------------------------");
    //第二步初始化数据源
    APPLICATION_CONTEXT.set::<Database>(init_mongdb(config).await);
    println!("---------------------------------------mongodb配置完成--------------------------------------------------");
    APPLICATION_CONTEXT.set::<Rbatis>(init_rbatis(config).await);
    println!("---------------------------------------mysql配置完成------------------------------------------------------");
    //第三步初始化所有的 服务类
    APPLICATION_CONTEXT.set::<ServiceContext>(ServiceContext::new());
    println!("---------------------------------------ServiceContext配置完成--------------------------------------------");
    //第三步初始化casbinCOntext
    APPLICATION_CONTEXT.set::<CasbinService>(CasbinService::default());
    println!("---------------------------------------CasbinService配置完成----------------------------------------------");
}
