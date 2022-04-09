pub mod event_service;
use cassie_domain::entity::log::{SysLogLogin, SysLogOperation};
use pharos::SharedPharos;

use crate::{observe::event::CassieEvent, service::crud_service::CrudService, APPLICATION_CONTEXT};

use super::log::log_service::{LogLoginService, LogOperationService};

//事件消费 待二次开发 todo
pub async fn consume(e: CassieEvent) {
    //在这里是获取不到 thread_local 的值 异步消费过来 已经不在同一个线程里了
    match e {
        //登录事件
        CassieEvent::LogLogin(dto) => {
            let mut entity = dto.into();
            let log_login_service = APPLICATION_CONTEXT.get::<LogLoginService>();
            log_login_service.save(&mut entity).await;
        }
        //操作事件
        CassieEvent::LogOperation(dto) => {
            let mut entity = dto.into();
            let log_operation_service = APPLICATION_CONTEXT.get::<LogOperationService>();
            log_operation_service.save(&mut entity).await;
        }
        //消息事件
        CassieEvent::Sms { sms_type } => {}
        //自定义事件
        CassieEvent::Custom { event_type, data } => todo!(),
    }
}
//发布事件
pub async fn fire_event(e: CassieEvent) {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    pharos.notify(e).await;
}
