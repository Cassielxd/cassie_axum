#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

pub mod config;
pub mod dao;

pub mod routers;
pub mod service;
pub mod cici_casbin;
pub mod middleware;
pub mod entity;
pub mod dto;
pub mod vo;
pub mod admin;
pub mod request;
pub mod nacos;

use request::*;
use std::sync::Arc;
use thread_local::ThreadLocal;

use crate::{config::config::ApplicationConfig, service::ServiceContext};
//初始化静态上下文
lazy_static! {
    pub static ref CASSIE_CONFIG: ApplicationConfig = ApplicationConfig::default();
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
    pub static ref REQUEST_CONTEXT: Arc<ThreadLocal<RequestModel>> =
    Arc::new(ThreadLocal::default());
}

