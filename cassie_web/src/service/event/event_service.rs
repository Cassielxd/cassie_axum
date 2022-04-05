use crate::observe::event::CassieEvent;

//事件核心处理类
pub struct EventService {}

impl EventService {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn consume(&self, e: CassieEvent) {
        match e {
            CassieEvent::Log {} => {
                println!("log event")
            }
            CassieEvent::Sms { sms_type } => {}
            CassieEvent::Custom { event_type, data } => {}
        }
    }
}
