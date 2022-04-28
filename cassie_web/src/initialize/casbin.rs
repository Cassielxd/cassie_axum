use crate::{CasbinService, APPLICATION_CONTEXT};

pub async fn init_casbin() {
  APPLICATION_CONTEXT.set::<CasbinService>(CasbinService::default().await);
}
