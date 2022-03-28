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
pub mod utils;
pub mod vo;
use mongodb::Database;
use rbatis::rbatis::Rbatis;
use request::*;

use crate::{config::config::ApplicationConfig, service::ServiceContext};
use state::Container;
//初始化静态上下文延迟加载
lazy_static! {
    pub static ref RB: Rbatis =
        async_std::task::block_on(async { crate::dao::init_rbatis().await });
}
pub static CONTAINER: Container![Send + Sync] = <Container![Send + Sync]>::new();

pub fn init_context() {
    CONTAINER.set::<ApplicationConfig>(ApplicationConfig::default());
    CONTAINER.set::<Database>(async_std::task::block_on(async {
        crate::dao::init_mongdb().await
    }));
    CONTAINER.set::<Rbatis>(async_std::task::block_on(async {
        crate::dao::init_rbatis().await
    }));
    CONTAINER.set::<ServiceContext>(ServiceContext::default());
}
