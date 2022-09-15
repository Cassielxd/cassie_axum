use cassie_common::error::{Error, Result};
pub trait ISmsService {}

pub struct SmsService {
    pub inner: Box<dyn ISmsService>,
}

impl SmsService {}
