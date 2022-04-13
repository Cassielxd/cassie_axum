use crate::middleware::auth::get_local;
use crate::service::crud_service::CrudService;
use crate::service::sys_user_service::SysUserService;
use crate::APPLICATION_CONTEXT;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use cassie_common::error::Error;
use cassie_common::RespVO;
use cassie_domain::dto::sys_user_dto::SysUserDTO;
use cassie_domain::entity::PageData;
use cassie_domain::request::SysUserQuery;
use validator::Validate;

/**
 *method:/user/page
 *desc:用户查询分页
 *author:String
 *email:348040933@qq.com
 */
pub async fn page(arg: Option<Query<SysUserQuery>>) -> impl IntoResponse {
    let sys_user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    let arg = arg.unwrap();
    let vo = sys_user_service
        .page(
            &arg,
            PageData {
                page_no: arg.page().clone(),
                page_size: arg.limit().clone(),
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
    let sys_user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    let arg = arg.unwrap();
    let vo = sys_user_service.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}
/**
 *method:/user/:id
 *desc:用户查询user
 *author:String
 *email:348040933@qq.com
 */
pub async fn get_user_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let sys_user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    let vo = sys_user_service.get(id).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn info() -> impl IntoResponse {
    let sys_user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    let request_model = get_local().unwrap();
    let mut vo = sys_user_service
        .get(request_model.uid().clone().to_string())
        .await
        .unwrap();
    vo.set_password(None);
    RespVO::from(&vo).resp_json()
}

/**
 *method:/user/save
 *desc:用户保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysUserDTO>) -> impl IntoResponse {
    let sys_user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    let user = arg;
    if let Err(e) = user.validate() {
        return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
    }

    sys_user_service.save_info(user).await;

    return RespVO::from(&"保存成功".to_string()).resp_json();
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    let sys_user_service = APPLICATION_CONTEXT.get::<SysUserService>();
    sys_user_service.delete_user(id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}
pub fn init_router() -> Router {
    Router::new()
        .route("/user", get(page).post(save).put(save))
        .route("/user/info", get(info))
        .route("/user/list", get(list))
        .route("/user/:id", get(get_user_by_id).delete(delete))
}
