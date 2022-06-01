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
use deno_core::{
    serde_v8,
    v8::{self},
};
use deno_runtime::worker::MainWorker;
use log::info;
use pharos::SharedPharos;
use retry::{delay::Fixed, retry_with_index, Error, OperationResult};
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
        CassieEvent::Sms { sms_type } => {
            //todo!("消息事件");  短信发送  公众号消息  app消息
        }
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
        //如果错误需要重试
        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > event.need_persist().unwrap() {
                return OperationResult::Err(format!("超过重试次数：{}", current_try));
            }
            match do_execute_script(workers, event.event_name().clone().unwrap().as_str(), code.as_str()) {
                Ok(result) => OperationResult::Ok(result),
                Err(e) => OperationResult::Retry(e.to_string()),
            }
        });
        handle_result(result, event.clone(), custom.clone());
    }

    workers.run_event_loop(false).await;
    info!("execute script time {} 毫秒", start.elapsed().as_millis().to_string());
}

//处理结果集  event 事件  custom参数
fn handle_result(result: Result<serde_json::Value, Error<String>>, event: EventConfigDTO, custom: CustomEvent) {
    match result {
        Ok(data) => {
            //处理结果 由于是异步处理处理结果可以放到数据库里
        }
        Err(e) => {
            //处理错误 保存错误日志
        }
    }
}
//执行脚本 并处理结果集
fn do_execute_script(workers: &mut MainWorker, name: &str, source_code: &str) -> Result<serde_json::Value, String> {
    match workers.js_runtime.execute_script(name, source_code) {
        Ok(global) => {
            //处理结果集
            let scope = &mut workers.js_runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            //把v8结果转换成 serde_json::Value
            let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);
            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(format!("Cannot deserialize value: {:?}", err)),
            }
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}

//构建脚本每个脚本独立运行上下文隔离
pub fn build_script(init_code: String, event_script: String) -> String {
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
