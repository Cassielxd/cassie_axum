use async_std::prelude::StreamExt;
use log::info;
use pharos::{Channel, Filter, ObserveConfig, SharedPharos};
use std::thread;

use crate::service::{consume_script, consume_sys};
use crate::{initialize::rules::init, CustomEvent, APPLICATION_CONTEXT};

use super::event::CassieEvent;

//事件消费处理类
pub async fn init_sys_consumer() {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    let filter = Filter::Pointer(|e| match e {
        CassieEvent::Custom(_) => {
            return false;
        }
        _ => {
            return true;
        }
    });
    let opts = ObserveConfig::from(filter).channel(Channel::Unbounded);
    let mut events = pharos.observe_shared(opts).await.unwrap();
    loop {
        let event = events.next().await.unwrap();
        consume_sys(event).await;
    }
}

pub async fn init_script_consumer() {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    let filter = Filter::Pointer(|e| match e {
        CassieEvent::Custom(_) => {
            return true;
        }
        _ => {
            return false;
        }
    });
    let opts = ObserveConfig::from(filter).channel(Channel::Unbounded);
    let mut events = pharos.observe_shared(opts).await.unwrap();
    let mut workers = init(None).await;
    loop {
        let event = events.next().await.unwrap();
        if let CassieEvent::Custom(cus) = event {
            consume_script(&mut workers, cus).await;
        }
    }
}
