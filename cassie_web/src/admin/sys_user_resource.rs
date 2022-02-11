use crate::{
    dto::sys_user_dto::SysUserDTO,
    entity::PageData,
    request::SysUserQuery,
    service::{crud_service::CrudService, CONTEXT},
};
use cassie_common::RespVO;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use axum::routing::post;
use cassie_common::error::Error;
use validator::Validate;
use axum::extract::{Path, Query};

/**
 *method:/user/page
 *desc:用户查询分页
 *author:String
 *email:348040933@qq.com
 */
pub async fn page(arg: Option<Query<SysUserQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .sys_user_service
        .page(
            &arg,
            PageData {
                page_no: arg.page_no.clone(),
                page_size: arg.page_size.clone(),
            },
        )
        .await;
    RespVO::from_result(&vo).resp_json()
}

/**
 *method:/user/list
 *desc:用户查询分页
 *author:String
 *email:348040933@qq.com
 */
pub async fn list(arg: Option<Query<SysUserQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT.sys_user_service.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn get_user_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let vo = CONTEXT.sys_user_service.get(id).await;
    RespVO::from_result(&vo).resp_json()
}

/**
 *method:/user/save
 *desc:用户保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysUserDTO>) -> impl IntoResponse {
    let user = arg;
    if let Err(e) = user.validate() {
        return RespVO::<()>::from_error("-1", &Error::E(e.to_string())).resp_json();
    }
    CONTEXT.sys_user_service.save_info(user).await;


    return RespVO::from(&"保存成功".to_string()).resp_json();
}


pub fn init_router() -> Router {
    Router::new()
        .route("/", get(page))
        .route("/list", get(list))
        .route("/save", post(save))
        .route("/:id", get(get_user_by_id))
}