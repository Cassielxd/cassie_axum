use serde::{Deserialize, Serialize};

pub use request_model::*;

use crate::vo::jwt::Resource;

pub mod request_model;
pub mod tree;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct RequestModel {
    uid: i64,
    super_admin: i32,
    username: String,
    agency_code: String,
    product_code: String,
    path: String,
    from: Resource,
}
