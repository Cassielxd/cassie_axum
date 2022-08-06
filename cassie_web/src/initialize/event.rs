use std::thread;

use crate::observe::consumer::{init_script_consumer, init_sys_consumer};
use crate::{CassieEvent, APPLICATION_CONTEXT};
use pharos::SharedPharos;

//初始化 event bus事件处理器
pub async fn init_event_bus() {
    APPLICATION_CONTEXT.set::<SharedPharos<CassieEvent>>(SharedPharos::default());
    //这里只能使用thread::spawn 多个监听者
    tokio::task::spawn(init_sys_consumer());
    let builder2 = thread::Builder::new().name("init_script_consumer".into());
    builder2.spawn(|| {
        async_std::task::block_on(async { init_script_consumer().await });
    });
}
