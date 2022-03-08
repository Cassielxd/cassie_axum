pub mod request_model;
pub use request_model::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct RequestModel {
    pub uid: i64,
    pub super_admin:i32,
    pub username: String,
    pub agency_code: String,
    pub product_code: String,
}
