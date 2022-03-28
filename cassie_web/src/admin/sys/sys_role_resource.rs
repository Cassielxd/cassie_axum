use crate::cici_casbin::casbin_service::CasbinService;
use crate::dto::sys_role_dto::SysRoleDTO;
use crate::request::SysRoleQuery;
use crate::service::ServiceContext;
use crate::CONTAINER;
use crate::{entity::PageData, service::crud_service::CrudService};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use casbin::MgmtApi;
use cassie_common::RespVO;

/**
 *method:/role/page
 *desc:角色分页查询
 *author:String
 *email:348040933@qq.com
 */
pub async fn page(arg: Option<Query<SysRoleQuery>>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    let arg = arg.unwrap();
    let vo = context
        .sys_role_service
        .page(
            &arg,
            PageData {
                page_no: arg.page.clone(),
                page_size: arg.limit.clone(),
            },
        )
        .await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn list(arg: Option<Query<SysRoleQuery>>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    let arg = arg.unwrap();
    let vo = context.sys_role_service.list(&arg).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    let mut vo = context.sys_role_service.get(id).await.unwrap();
    let menu_list = context
        .sys_role_service
        .sys_role_menu_service
        .get_menu_id_list(vo.id.clone().unwrap())
        .await;
    vo.menuid_list = Option::from(menu_list);
    RespVO::from(&vo).resp_json()
}

pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    context.sys_role_service.delete_by_role_id(id).await;
    RespVO::from(&"删除成功".to_string()).resp_json()
}

/**
 *method:/role/save
 *desc:角色保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysRoleDTO>) -> impl IntoResponse {
    let context = CONTAINER.get::<ServiceContext>();
    context.sys_role_service.save_role(arg).await;
    RespVO::from(&"保存成功".to_string()).resp_json()
}

/**
 *method:/role/casbin_test
 *desc:casbin test
 *author:String
 *email:348040933@qq.com
 */
pub async fn casbin_test() -> impl IntoResponse {
    let rules = vec![
        vec![
            "admin".to_owned(),
            "superadmin".to_owned(),
            "/cici_admin/user/list".to_owned(),
            "POST".to_owned(),
        ],
        vec![
            "admin1".to_owned(),
            "superadmin".to_owned(),
            "/cici_admin/user/list".to_owned(),
            "POST".to_owned(),
        ],
        vec![
            "admin2".to_owned(),
            "superadmin".to_owned(),
            "/cici_admin/user/list".to_owned(),
            "POST".to_owned(),
        ],
        vec![
            "admin3".to_owned(),
            "superadmin".to_owned(),
            "/cici_admin/user/list".to_owned(),
            "POST".to_owned(),
        ],
    ];
    let cached_enforcer = CONTAINER.get::<CasbinService>().enforcer.clone();
    let mut enforcer = cached_enforcer.write().await;
    enforcer.add_policies(rules).await;

    let user_rules = vec![
        vec![
            "lixingdong1".to_owned(), //username
            "admin".to_owned(),       //role_code
            "superadmin".to_owned(),  //anency_code
        ],
        vec![
            "lixingdong2".to_owned(),
            "admin".to_owned(),
            "superadmin".to_owned(),
        ],
        vec![
            "lixingdong3".to_owned(),
            "admin".to_owned(),
            "superadmin".to_owned(),
        ],
        vec![
            "lixingdong4".to_owned(),
            "admin".to_owned(),
            "superadmin".to_owned(),
        ],
    ];
    enforcer.add_grouping_policies(user_rules).await;
    let res = Ok("保存成功".to_string());
    RespVO::from_result(&res).resp_json()
}

pub fn init_router() -> Router {
    Router::new()
        .route("/role", get(page).post(save).put(save))
        .route("/role/:id", get(get_by_id).delete(delete))
        .route("/role/list", get(list))
}
