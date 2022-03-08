use crate::entity::sys_entitys::CommonField;
use crate::{
    dto::sys_dict_dto::{SysDictDataDTO, SysDictTypeDTO},
    entity::sys_entitys::{SysDictData, SysDictType},
    request::SysDictQuery,
};
use crate::{CONTEXT, RB};

use super::crud_service::CrudService;

/**
*struct:SysDictTypeService
*desc:定义type处理逻辑
*author:String
*email:348040933@qq.com
*/
pub struct SysDictTypeService {}

impl SysDictTypeService {
    pub async fn get_all_list(&self) {
        let q = SysDictQuery {
            id: None,
            dict_type_id: None,
            dict_type: None,
            dict_name: None,
            page_no: None,
            page_size: None,
        };
        let dict = self.list(&q).await;
        let dict_value = CONTEXT.sys_dict_value_service.list(&q).await;
        if let Ok(mut dlist) = dict {
            for mut d in dlist {
                if let Ok(ref dva) = dict_value {
                    for dv in dva {
                        if d.id == dv.dict_type_id {
                            //添加到
                          
                        }
                    }
                }
            }
        }
    }
}

impl Default for SysDictTypeService {
    fn default() -> Self {
        SysDictTypeService {}
    }
}
impl CrudService<SysDictType, SysDictTypeDTO, SysDictQuery> for SysDictTypeService {
    fn get_wrapper(arg: &SysDictQuery) -> rbatis::wrapper::Wrapper {
        RB.new_wrapper()
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
impl Default for SysDictDataService {
    fn default() -> Self {
        SysDictDataService {}
    }
}
impl CrudService<SysDictData, SysDictDataDTO, SysDictQuery> for SysDictDataService {
    fn get_wrapper(arg: &SysDictQuery) -> rbatis::wrapper::Wrapper {
        RB.new_wrapper()
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut SysDictData) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
