use crate::{
    dto::asi_dto::AsiGroupColumnDTO, entity::PageData, request::AsiQuery,
    service::crud_service::CrudService, CONTEXT,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json, Router, routing::get,
};
use cassie_common::{error::Error, RespVO};
use validator::Validate;

pub async fn page(arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .asi_service
        .asi_column
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
pub async fn list(Path(group_code): Path<String>,arg: Option<Query<AsiQuery>>) -> impl IntoResponse {
    let res= CONTEXT.asi_service.asi_column.fetch_list_by_column("group_code", &vec![group_code]).await;
    RespVO::from_result(&res).resp_json()
}

pub async fn save(
    Path(group_code): Path<String>,
    Json(arg): Json<Vec<AsiGroupColumnDTO>>,
) -> impl IntoResponse {
    /*验证是否存在业务分类*/
    let group = CONTEXT.asi_service.fetch_list_by_column("group_code", &vec![group_code]).await;
    match group{
        Ok(list) => {
            let group_info = list.get(0);
            /*执行验证逻辑*/
            for dto in &arg {
                if let Err(e) = dto.validate() {
                    return RespVO::<()>::from_error(&Error::E(e.to_string())).resp_json();
                }
            }
            /*执行保存逻辑*/
            CONTEXT
                .asi_service
                .asi_column
                .save_batch_colums(group_info.unwrap().clone(), arg)
                .await;
            RespVO::from_result(&Ok("保存成功".to_string())).resp_json()
        },
        Err(_) => {
            return RespVO::<()>::from_error(&Error::E("业务分类没有定义".to_string())).resp_json();
        },
    }

   
}
pub fn init_router()->Router{
    Router::new().route(
        "/asi/column/list/:group_code",
        get(list)
            .post(save)
            .put(save),
    )
}