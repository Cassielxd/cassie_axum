use axum::Router;

use crate::api::{user_resources, wxapp_resources};

//api
pub fn routers() -> Router {
  Router::new()
    .merge(wxapp_resources::init_router())
    .merge(user_resources::init_router())
}
