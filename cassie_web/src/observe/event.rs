use std::collections::HashMap;

use cassie_domain::dto::sys_log::{SysLogLoginDto, SysLogOperationDto};

#[derive(Clone, Debug, PartialEq)]
pub enum CassieEvent {
    LogLogin(SysLogLoginDto),
    LogOperation(SysLogOperationDto),
    Sms {
        sms_type: u8,
    },
    Custom {
        event_type: u8, // 1 脚本 2 其他业务分类(待定)
        data: HashMap<String, String>,
    },
}
