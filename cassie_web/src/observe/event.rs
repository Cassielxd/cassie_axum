use std::collections::HashMap;

use cassie_domain::dto::sys_log::{SysLogLoginDto, SysLogOperationDto};
use cassie_domain::request::RequestModel;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomEvent {
    pub path: String,
    pub params_values: Option<HashMap < String, serde_json :: Value >>,
    pub return_values: serde_json::Value,
    pub request_model: Option<RequestModel>,
}

impl CustomEvent {
    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&json!(self)).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CassieEvent {
    LogLogin(SysLogLoginDto),
    LogOperation(SysLogOperationDto),
    Sms { sms_type: u8 },
    Custom(CustomEvent),
}
