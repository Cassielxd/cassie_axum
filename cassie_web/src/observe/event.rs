use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum CassieEvent {
    Log {},
    Sms {
        sms_type: u8,
    },
    Custom {
        event_type: u8, // 1 脚本 2 其他业务分类(待定)
        data: HashMap<String, String>,
    },
}
