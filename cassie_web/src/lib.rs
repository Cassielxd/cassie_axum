#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
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
pub mod vo;
use request::*;
use std::sync::Arc;
use thread_local::ThreadLocal;

use crate::{config::config::ApplicationConfig, service::ServiceContext};
//初始化静态上下文延迟加载
lazy_static! {
    //环境配置
    pub static ref CASSIE_CONFIG: ApplicationConfig = ApplicationConfig::default();
    //service服务类
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
    //登录信息透传
    pub static ref REQUEST_CONTEXT: Arc<ThreadLocal<RequestModel>> =
        Arc::new(ThreadLocal::default());
}
