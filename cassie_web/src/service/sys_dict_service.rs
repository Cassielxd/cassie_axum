use crate::CONTEXT;
use crate::entity::sys_entitys::CommonField;
use crate::{
    dto::sys_dict_dto::{SysDictDataDTO, SysDictTypeDTO},
    entity::sys_entitys::{SysDictData, SysDictType},
    request::SysDictQuery,
};

use super::{crud_service::CrudService};

/**
*struct:SysDictTypeService
*desc:定义type处理逻辑
*author:String
*email:348040933@qq.com
*/
pub struct SysDictTypeService {}

impl  Default for SysDictTypeService {
    fn default() -> Self {
        SysDictTypeService{}
    }
}
impl CrudService<SysDictType, SysDictTypeDTO, SysDictQuery> for SysDictTypeService {
    fn get_wrapper(arg: &SysDictQuery) -> rbatis::wrapper::Wrapper {
        CONTEXT.rbatis.new_wrapper()
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut SysDictType) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}

/**
*struct:
*desc:定义value处理逻辑
*author:String
*email:348040933@qq.com
*/
pub struct SysDictDataService {}
impl  Default for SysDictDataService {
    fn default() -> Self {
        SysDictDataService{}
    }
}
impl CrudService<SysDictData, SysDictDataDTO, SysDictQuery> for SysDictDataService {
    fn get_wrapper(arg: &SysDictQuery) -> rbatis::wrapper::Wrapper {
        CONTEXT.rbatis.new_wrapper()
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut SysDictData) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
