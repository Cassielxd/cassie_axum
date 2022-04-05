use pharos::SharedPharos;

use crate::{observe::event::CassieEvent, APPLICATION_CONTEXT};

//事件消费 待二次开发 todo
pub async fn consume(e: CassieEvent) {
    match e {
        CassieEvent::Log {} => {
            todo!("日志处理");
        }
        CassieEvent::Sms { sms_type } => {
            todo!("消息事件处理")
        }
        CassieEvent::Custom { event_type, data } => {
            todo!("自定义事件处理")
        }
    }
}
//发布事件
pub async fn fire_event(e: CassieEvent) {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    pharos.notify(e).await;
}
