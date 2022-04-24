pub mod event_service;
pub mod ops;
use casbin::function_map::key_match2;
use cassie_domain::dto::sys_event_dto::EventConfigDTO;
use log::info;
use pharos::SharedPharos;
use serde_json::json;

use crate::{
    initialize::rules::init,
    observe::event::{CassieEvent, CustomEvent},
    service::crud_service::CrudService,
    APPLICATION_CONTEXT,
};

use self::event_service::EventConfigService;

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
        CassieEvent::Sms { sms_type } => todo!("待开发"),
        //自定义事件
        CassieEvent::Custom(custom) => {
            let event_config_service = APPLICATION_CONTEXT.get::<EventConfigService>();
            //获取到所有的事件配置
            let list = event_config_service.load_event().await;
            if let Ok(data) = list {
                let d = data
                    .iter()
                    .filter(|item| {
                        key_match2(&custom.path.clone().as_str(), &item.path().clone().unwrap())
                            || item.path().clone().unwrap().contains(&custom.path.clone())
                    })
                    .collect::<Vec<_>>();

                if d.len() > 0 {
                    execute_script(d, &custom);
                }
            }
        }
    }
}
//核心动态脚本执行方法
fn execute_script(data: Vec<&EventConfigDTO>, custom: &CustomEvent) {
    let init_code = format!(
        r#" var request_context=JSON.parse({});"#,
        serde_json::to_string_pretty(&serde_json::to_string_pretty(&json!(custom)).unwrap())
            .unwrap()
    );
    let mut workers = init(None);
    workers.execute_script("init_request_context", &init_code);
    for event in data {
        match workers.js_runtime.execute_script(
            event.event_name().clone().unwrap().as_str(),
            event.event_script().clone().unwrap().as_str(),
        ) {
            Ok(data) => {}
            Err(e) => {
                info!("error info {:#?}", e.to_string());
            }
        }
    }
}

//发布事件
pub async fn fire_event(e: CassieEvent) {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    pharos.notify(e).await;
}
