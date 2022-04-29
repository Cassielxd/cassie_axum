use super::crud_service::CrudService;
use crate::APPLICATION_CONTEXT;
use cached::proc_macro::cached;
use cassie_common::error::Result;
use cassie_domain::entity::sys_entitys::CommonField;
use cassie_domain::{
    dto::sys_dict_dto::{SysDictDataDTO, SysDictTypeDTO},
    entity::sys_entitys::{SysDictData, SysDictType},
    request::SysDictQuery,
};
use rbatis::rbatis::Rbatis;

/**
*struct:SysDictTypeService
*desc:定义type处理逻辑
*author:String
*email:348040933@qq.com
*/
#[cached(name = "ALL_DICT_LIST", time = 180, result = true, size = 100)]
pub async fn get_all_list() -> Result<Vec<SysDictTypeDTO>> {
    let query = SysDictQuery::default();
    let sys_dict_type_service = APPLICATION_CONTEXT.get::<SysDictTypeService>();
    let sys_dict_value_service = APPLICATION_CONTEXT.get::<SysDictDataService>();
    let mut dict = sys_dict_type_service.list(&query).await?;
    let dict_value = sys_dict_value_service.list(&query).await?;
    for d in &mut dict {
        let mut data = vec![];
        for dv in &dict_value {
            if d.id() == dv.dict_type_id() {
                //添加到
                data.push(dv.clone());
            }
        }
        d.set_data_list(Option::Some(data));
    }
    Ok(dict)
}
pub struct SysDictTypeService {}

impl SysDictTypeService {
    //获取所有的type
}

impl Default for SysDictTypeService {
    fn default() -> Self {
        SysDictTypeService {}
    }
}
impl CrudService<SysDictType, SysDictTypeDTO, SysDictQuery> for SysDictTypeService {
    fn get_wrapper(arg: &SysDictQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
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
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper().do_if(arg.dict_type_id().is_some(), |w| w.eq(SysDictData::dict_type_id(), arg.dict_type_id()))
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut SysDictData) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
