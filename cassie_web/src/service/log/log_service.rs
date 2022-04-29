use crate::service::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use cassie_domain::dto::sys_log::{SysLogLoginDto, SysLogOperationDto};
use cassie_domain::entity::log::{SysLogLogin, SysLogOperation};
use cassie_domain::entity::sys_entitys::CommonField;
use cassie_domain::request::LogQuery;
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;

pub struct LogLoginService {}
impl Default for LogLoginService {
  fn default() -> Self {
    LogLoginService {}
  }
}
impl CrudService<SysLogLogin, SysLogLoginDto, LogQuery> for LogLoginService {
  fn get_wrapper(arg: &LogQuery) -> Wrapper {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    rb.new_wrapper()
  }

  fn set_save_common_fields(&self, common: CommonField, data: &mut SysLogLogin) {
    data.id = common.id;
    data.create_date = common.create_date;
  }
}
pub struct LogOperationService {}
impl CrudService<SysLogOperation, SysLogOperationDto, LogQuery> for LogOperationService {
  fn get_wrapper(arg: &LogQuery) -> Wrapper {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    rb.new_wrapper()
  }

  fn set_save_common_fields(&self, common: CommonField, data: &mut SysLogOperation) {
    data.id = common.id;
    data.create_date = common.create_date;
  }
}
impl Default for LogOperationService {
  fn default() -> Self {
    LogOperationService {}
  }
}
