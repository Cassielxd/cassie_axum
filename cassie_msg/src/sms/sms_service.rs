use cassie_common::error::{Error, Result};
pub trait ISmsService {
    fn sms_send(&self, mobiles: &vec![str], v: &str) -> Result<String>;
}

pub struct SmsService {
    pub inner: Box<dyn ISmsService>,
}

impl SmsService {}
