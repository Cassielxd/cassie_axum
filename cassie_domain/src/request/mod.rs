use serde::{Deserialize, Serialize};

pub use request_model::*;

use crate::vo::jwt::{Source, JWTToken};

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
    from: Source,
}

impl RequestModel {
   pub fn new(data: JWTToken, path: String) -> Self {
        let mut model=RequestModel::default();
        model.set_uid(data.id().clone());
        model.set_agency_code(data.agency_code().clone());
        model.set_super_admin(data.super_admin().clone());
        model.set_username(data.username().clone());
        model.set_path(path);
        model.set_from(data.from().clone());
        model
    }
}