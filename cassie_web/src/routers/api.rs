use axum::{middleware::from_extractor, Router};

use crate::{
    api::{user_resources, wxapp_resources},
    middleware::auth_api,
};

//api
pub fn routers() -> Router {
    need_auth_routers().merge(noneed_auth_routers())
}
//需要权限认证的路由
pub fn need_auth_routers() -> Router {
    Router::new().merge(user_resources::init_router()).layer(from_extractor::<auth_api::Auth>())
}

//不需要权限认证的路由
pub fn noneed_auth_routers() -> Router {
    Router::new().merge(wxapp_resources::init_router())
}
