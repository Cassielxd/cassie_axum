#![allow(non_snake_case)]
#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]
pub mod dto;
pub mod entity;
pub mod request;
pub mod vo;
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate getset;
