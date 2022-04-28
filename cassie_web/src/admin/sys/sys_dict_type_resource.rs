use crate::service::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use axum::routing::get;
use axum::Json;
use axum::{response::IntoResponse, Router};
use cassie_common::RespVO;

use crate::service::sys_dict_service::{get_all_list, SysDictTypeService};
use axum::extract::{Path, Query};
use cassie_domain::dto::sys_dict_dto::SysDictTypeDTO;
use cassie_domain::entity::PageData;
use cassie_domain::request::SysDictQuery;

/**
 *method:/dict/type/page
 *desc:数据字典 分页查询
 *author:String
 *email:348040933@qq.com
 */

pub async fn page(arg: Option<Query<SysDictQuery>>) -> impl IntoResponse {
  let sys_dict_type_service = APPLICATION_CONTEXT.get::<SysDictTypeService>();
  let arg = arg.unwrap();
  let vo = sys_dict_type_service
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
 *method:/dict/type/{id}
 *desc:数据字典id获取
 *author:String
 *email:348040933@qq.com
 */
pub async fn get_by_id(Path(id): Path<String>) -> impl IntoResponse {
  let sys_dict_type_service = APPLICATION_CONTEXT.get::<SysDictTypeService>();
  let dto = sys_dict_type_service.get(id).await;
  RespVO::from_result(&dto).resp_json()
}

/**
 *method:/dict/type/save
 *desc:数据字典保存
 *author:String
 *email:348040933@qq.com
 */
pub async fn save(Json(arg): Json<SysDictTypeDTO>) -> impl IntoResponse {
  let sys_dict_type_service = APPLICATION_CONTEXT.get::<SysDictTypeService>();
  let mut entity = arg.into();
  let vo = sys_dict_type_service.save(&mut entity).await;
  RespVO::from_result(&vo).resp_json()
}

pub async fn edit(Json(arg): Json<SysDictTypeDTO>) -> impl IntoResponse {
  let sys_dict_type_service = APPLICATION_CONTEXT.get::<SysDictTypeService>();
  let id = arg.id().clone();
  let mut entity = arg.into();
  sys_dict_type_service
    .update_by_id(id.unwrap().to_string(), &mut entity)
    .await;
  RespVO::from(&"更新成功".to_string()).resp_json()
}
pub async fn delete(Path(id): Path<String>) -> impl IntoResponse {
  let sys_dict_type_service = APPLICATION_CONTEXT.get::<SysDictTypeService>();
  sys_dict_type_service.del(&id).await;
  RespVO::from(&"删除成功".to_string()).resp_json()
}

pub async fn all() -> impl IntoResponse {
  let vo = get_all_list().await;
  RespVO::from_result(&vo).resp_json()
}

pub fn init_router() -> Router {
  Router::new()
    .route("/dict/type/all", get(all))
    .route("/dict/type", get(page).post(save).put(edit))
    .route("/dict/type/:id", get(get_by_id).delete(delete))
}
