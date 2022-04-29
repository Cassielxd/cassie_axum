use async_std::prelude::StreamExt;
use pharos::{Channel, SharedPharos};

use crate::{service::consume, APPLICATION_CONTEXT};

use super::event::CassieEvent;

//事件消费处理类
pub async fn init_consumer() {
  let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
  let mut events = pharos.observe_shared(Channel::Unbounded.into()).await.unwrap();
  loop {
    let event = events.next().await.unwrap();
    consume(event).await;
  }
}
