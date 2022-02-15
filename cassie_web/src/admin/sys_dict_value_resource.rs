use crate::{
    entity::PageData,
    service::{crud_service::CrudService, CONTEXT},
};
use cassie_common::RespVO;
use axum::response::IntoResponse;
use axum::Json;


use axum::extract::{Path, Query};
use crate::request::SysDictQuery;
use crate::dto::sys_dict_dto::SysDictDataDTO;

/**
 *method:/dict/type/page
 *desc:数据字典 分页查询
 *author:String
 *email:348040933@qq.com
 */

pub async fn page(arg:  Option<Query<SysDictQuery>>) -> impl IntoResponse {
    let arg = arg.unwrap();
    let vo = CONTEXT
        .sys_dict_value_service
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
 *method:/dict/type/{id}
 *desc:数据字典id获取
 *author:String
 *email:348040933@qq.com
 */
pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let dto = CONTEXT.sys_dict_value_service.get(id).await;
    RespVO::from_result(&dto).resp_json()
}

/**
 *method:/dict/type/save
 *desc:数据字典保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysDictDataDTO>) -> impl IntoResponse {
    let mut entity = arg.into();
    let vo = CONTEXT.sys_dict_value_service.save(&mut entity).await;
    RespVO::from_result(&vo).resp_json()
}

