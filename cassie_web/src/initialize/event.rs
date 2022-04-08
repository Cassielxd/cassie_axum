use crate::{init_consumer, CassieEvent, APPLICATION_CONTEXT};
use pharos::SharedPharos;

//初始化 event bus事件处理器
pub async fn init_event_bus() {
    APPLICATION_CONTEXT.set::<SharedPharos<CassieEvent>>(SharedPharos::default());
    tokio::task::spawn(init_consumer());
}
