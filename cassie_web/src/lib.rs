#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate rbatis;

pub mod config;
pub mod dao;

pub mod admin;
pub mod cici_casbin;
pub mod dto;
pub mod entity;
pub mod middleware;
//nacos注册中心默认实现
pub mod nacos;
pub mod request;
pub mod routers;
pub mod service;
pub mod utils;
pub mod vo;
use mongodb::Database;
use rbatis::rbatis::Rbatis;
use request::*;

use crate::cici_casbin::casbin_service::CasbinService;
use crate::{config::config::ApplicationConfig, service::ServiceContext};
use state::Container;
pub static CONTAINER: Container![Send + Sync] = <Container![Send + Sync]>::new();
/*初始化环境上下文*/
pub fn init_context() {
    //第一步加载配置
    println!("--------------------------------------------------------正在启动--------------------------------------------------------");
    CONTAINER.set::<ApplicationConfig>(ApplicationConfig::default());
    println!("--------------------------------------------------------配置配置完成-----------------------------------------------------");
    //第二步初始化数据源
    CONTAINER.set::<Database>(async_std::task::block_on(async {
        crate::dao::init_mongdb().await
    }));
    println!("--------------------------------------------------------mongodb配置完成--------------------------------------------------");
    CONTAINER.set::<Rbatis>(async_std::task::block_on(async {
        crate::dao::init_rbatis().await
    }));
    println!("--------------------------------------------------------mysql配置完成------------------------------------------------------");
    //第三步初始化所有的 服务类
    CONTAINER.set::<ServiceContext>(ServiceContext::default());
    println!("--------------------------------------------------------ServiceContext配置完成--------------------------------------------");
    //第三步初始化casbinCOntext
    CONTAINER.set::<CasbinService>(CasbinService::default());
    println!("--------------------------------------------------------CasbinService配置完成----------------------------------------------");
}
