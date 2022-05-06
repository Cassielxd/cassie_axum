pub mod event_service;
pub mod ops;
use self::event_service::{load_event, EventConfigService};
use super::log::log_service::{LogLoginService, LogOperationService};
use crate::{
    observe::event::{CassieEvent, CustomEvent},
    service::crud_service::CrudService,
    APPLICATION_CONTEXT,
};
use casbin::function_map::key_match2;
use cassie_domain::dto::sys_event_dto::EventConfigDTO;
use deno_runtime::worker::MainWorker;
use log::info;
use pharos::SharedPharos;
use serde_json::json;
use tokio::time::Instant;
//事件消费 待二次开发 todo
pub async fn consume(worker: &mut MainWorker, e: CassieEvent) {
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
            let list = load_event().await;
            if let Ok(data) = list {
                let d = data
                    .iter()
                    .filter(|item| key_match2(&custom.path.clone().as_str(), &item.path().clone().unwrap()) || item.path().clone().unwrap().contains(&custom.path.clone()))
                    .collect::<Vec<_>>();
                info!("事件个数：{:?}", d.len());
                if d.len() > 0 {
                    execute_script(worker, d, &custom).await;
                }
            }
        }
    }
}

//核心动态脚本执行方法
async fn execute_script(workers: &mut MainWorker, data: Vec<&EventConfigDTO>, custom: &CustomEvent) {
    let start = Instant::now();
    let init_code = format!(r#" var request_context={};"#, custom.as_json());
    for event in data {
        let code = build_script(init_code.clone(), event.event_script().clone().unwrap().clone());
        match workers.js_runtime.execute_script(event.event_name().clone().unwrap().as_str(), code.as_str()) {
            Ok(data) => {}
            Err(e) => {
                info!("error info {:#?}", e.to_string());
            }
        }
    }
    workers.run_event_loop(false).await;
    info!("execute script time {} 毫秒", start.elapsed().as_millis().to_string());
}
//构建脚本每个脚本独立运行上下文隔离
fn build_script(init_code: String, event_script: String) -> String {
    let script = format!(
        r#"
        "use strict";
        ((window) => {{
         {}
         {}
        }})(this);
    "#,
        init_code, event_script
    );
    script
}

//发布事件
pub async fn fire_event(e: CassieEvent) {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    pharos.notify(e).await;
}
