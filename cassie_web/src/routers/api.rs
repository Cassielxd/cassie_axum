use axum::Router;

use crate::api::wxapp_resources;

//api
pub fn routers() -> Router {
    Router::new().merge(wxapp_resources::init_router())
}
