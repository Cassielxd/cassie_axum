pub mod event_service;
use cassie_domain::entity::log::{SysLogLogin, SysLogOperation};
use pharos::SharedPharos;

use crate::{
    observe::event::CassieEvent,
    service::{crud_service::CrudService, ServiceContext},
    APPLICATION_CONTEXT,
};

//事件消费 待二次开发 todo
pub async fn consume(e: CassieEvent) {
    let service = APPLICATION_CONTEXT.get::<ServiceContext>();
    //在这里是获取不到 thread_local 的值 异步消费过来 已经不在同一个线程里了
    match e {
        //登录事件
        CassieEvent::LogLogin {
            operation,
            user_agent,
            ip,
            creator_name,
            creator,
        } => {
            let mut entity = SysLogLogin {
                id: None,
                operation,
                user_agent,
                ip,
                creator_name,
                creator,
                create_date: None,
            };
            service.log_login_service.save(&mut entity).await;
        }
        //操作事件
        CassieEvent::LogOperation {
            operation,
            request_uri,
            ip,
            creator_name,
            request_params,
            request_method,
            request_time,
            status,
        } => {
            let mut entity = SysLogOperation {
                id: None,
                operation,
                request_uri,
                request_params,
                request_method,
                request_time,
                status,
                ip,
                creator_name,
                creator: None,
                create_date: None,
            };
            service.log_operation_service.save(&mut entity).await;
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
