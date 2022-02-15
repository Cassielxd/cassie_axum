use crate::{
    entity::PageData,
    service::{crud_service::CrudService, CONTEXT},
};
use crate::request::SysRoleQuery;
use axum::extract::{Query, Path};
use cassie_common::RespVO;
use crate::dto::sys_role_dto::SysRoleDTO;
use axum::{Json};
use axum::response::IntoResponse;
use crate::cici_casbin::CASBIN_CONTEXT;
use casbin::MgmtApi;
use cassie_common::error::Error;
use validator::Validate;

/**
 *method:/role/page
 *desc:角色分页查询
 *author:String
 *email:348040933@qq.com
 */
pub async fn page(arg: Option<Query<SysRoleQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .sys_role_service
        .page(
            &arg,
            PageData {
                page_no: arg.0.page_no.clone(),
                page_size: arg.0.page_size.clone(),
            },
        )
        .await;
    RespVO::from_result(&vo).resp_json()
}
pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let vo = CONTEXT.sys_role_service.get(id).await;
    RespVO::from_result(&vo).resp_json()
}

pub async fn edit(Path(id): Path<String>,Json(arg): Json<SysRoleDTO>) -> impl IntoResponse {
    let role = arg;
    if let Err(e) = role.validate() {
       return RespVO::<()>::from_error("-1", &Error::E(e.to_string())).resp_json();
    }
    CONTEXT.sys_role_service.update_by_id(id,&role.into()).await;
    RespVO::from(&"修改成功".to_string()).resp_json()
}
/**
 *method:/role/save
 *desc:角色保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysRoleDTO>) -> impl IntoResponse {
    CONTEXT.sys_role_service.save_role(arg).await;
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
    let cached_enforcer = CASBIN_CONTEXT.enforcer.clone();
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

