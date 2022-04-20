use std::collections::HashMap;

use cassie_domain::dto::sys_log::{SysLogLoginDto, SysLogOperationDto};
use cassie_domain::request::RequestModel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomEvent {
    pub path: String,
    pub params_values: Option<HashMap<String, String>>,
    pub return_values: Option<HashMap<String, String>>,
    pub request_model: Option<RequestModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CassieEvent {
    LogLogin(SysLogLoginDto),
    LogOperation(SysLogOperationDto),
    Sms { sms_type: u8 },
    Custom(CustomEvent),
}
