use crate::sms::sms_service::ISmsService;

pub struct AliDaYuSmsService {}
impl ISmsService for AliDaYuSmsService {
    fn sms_send(&self, mobiles: &vec![str], v: &str) -> cassie_common::error::Result<String> {
        todo!()
    }
}
