use std::collections::HashMap;

use cassie_domain::request::RequestModel;

#[derive(Clone, Debug, PartialEq)]
pub enum CassieEvent {
    LogLogin {
        operation: Option<String>,
        user_agent: Option<String>,
        ip: Option<String>,
        creator_name: Option<String>,
        creator: Option<i64>,
    },
    LogOperation {
        operation: Option<String>,
        request_uri: Option<String>,
        ip: Option<String>,
        creator_name: Option<String>,
        request_params: Option<String>,
        request_method: Option<String>,
        request_time: Option<String>,
        status: Option<i8>,
    },
    Sms {
        sms_type: u8,
    },
    Custom {
        event_type: u8, // 1 脚本 2 其他业务分类(待定)
        data: HashMap<String, String>,
    },
}
