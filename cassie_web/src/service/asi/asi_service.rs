use crate::dto::asi_dto::{AsiGroupColumnDTO, AsiGroupDTO, AsiGroupValuesDTO};
use crate::entity::asi_entitys::{AsiGroup, AsiGroupColumn, AsiGroupValues};
use crate::entity::sys_entitys::CommonField;
use crate::service::crud_service::CrudService;
use crate::{AsiQuery, RB, REQUEST_CONTEXT};

use cassie_common::error::Error;
use cassie_common::error::Result;
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;

use super::asi_validation::validate_values;
/**
 *struct:AsiGroupService
 *desc:动态表单基础service
 *author:String
 *email:348040933@qq.com
 */
pub struct AsiGroupService {
    pub asi_column: AsiGroupColumnService,
    pub asi_values: AsiGroupValuesService,
}

impl AsiGroupService {
    /**
     *method:save_group
     *desc:保存业务分组
     *author:String
     *email:348040933@qq.com
     */
    pub async fn save_group(&self, group: AsiGroupDTO) -> Result<i64> {
        /*查询有没有重复的*/
        let g = group.group_code.clone();
        let count = crate::RB
            .fetch_count_by_wrapper::<AsiGroup>(
                RB.new_wrapper().eq(AsiGroup::group_code(), g.unwrap()),
            )
            .await;
        if let Ok(c) = count {
            return Err(cassie_common::error::Error::from(
                "group_code已经存在".to_string(),
            ));
        }
        let tls = REQUEST_CONTEXT.clone();
        let a = tls.get().unwrap();
        let mut entity: AsiGroup = group.into();
        entity.agency_code = Some(a.agency_code.clone());
        self.save(&mut entity).await
    }

    pub async fn save_values(
        &self,
        group_code: String,
        args: Vec<AsiGroupValuesDTO>,
    ) -> Result<bool> {
        let query = AsiQuery {
            column_code: None,
            group_code: Option::Some(group_code),
            page: None,
            limit: None,
            order: None,
            order_field: None,
        };

        //获取分组定义信息
        let group = self.list(&query).await;

        match group {
            Ok(data) => {
                //获取定义列信息
                let columns = self.asi_column.list(&query).await;
                match columns {
                    Ok(cloums_list) => {
                        ///验证数据定义
                        match validate_values(&cloums_list, &args) {
                            Ok(_) => {
                                //验证通过 保存数据
                                self.asi_values.save_batch_values(args).await;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    Err(_) => {
                        return Err(Error::from("列定义不存在!"));
                    }
                }
            }
            Err(_) => {
                return Err(Error::from("业务定义不存在!"));
            }
        }

        return Ok(true);
    }
}

impl Default for AsiGroupService {
    fn default() -> Self {
        AsiGroupService {
            asi_column: AsiGroupColumnService::default(),
            asi_values: AsiGroupValuesService::default(),
        }
    }
}

impl CrudService<AsiGroup, AsiGroupDTO, AsiQuery> for AsiGroupService {
    fn get_wrapper(arg: &AsiQuery) -> Wrapper {
        let mut wrapper = RB.new_wrapper();
        if arg.group_code.is_some() {
            wrapper = wrapper.eq(AsiGroup::group_code(), arg.group_code.clone().unwrap());
        }
        wrapper
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut AsiGroup) {
        data.id = common.id;
    }
}

/**------------------------------------------------------------------------------------------------------------------
*struct:AsiGroupColumnService
*desc:Column 定义service
*author:String
*email:348040933@qq.com
*/
pub struct AsiGroupColumnService {}

impl AsiGroupColumnService {
    /**
     *method:save_batch_colums
     *desc: 保存列定义
     *author:String
     *email:348040933@qq.com
     */
    pub async fn save_batch_colums(&self, group: AsiGroupDTO, columns: Vec<AsiGroupColumnDTO>) {
        /*不管是不是存在 直接删除在新增*/
        let group_code = group.group_code;
        self.del_by_column(
            AsiGroupColumn::group_code(),
            group_code.clone().unwrap().as_str(),
        )
        .await;
        /*获取当前登录信息*/
        let tls = REQUEST_CONTEXT.clone();
        let a = tls.get().unwrap();

        let mut entitys = vec![];
        /*构造信息*/
        for column in columns {
            let mut e: AsiGroupColumn = column.into();
            e.group_code = group_code.clone();
            e.agency_code = Some(a.agency_code.clone());
            entitys.push(e);
        }
        self.save_batch(&mut entitys).await;
    }
}

impl Default for AsiGroupColumnService {
    fn default() -> Self {
        AsiGroupColumnService {}
    }
}

impl CrudService<AsiGroupColumn, AsiGroupColumnDTO, AsiQuery> for AsiGroupColumnService {
    fn get_wrapper(arg: &AsiQuery) -> Wrapper {
        let mut wrapper = RB.new_wrapper();
        if arg.group_code.is_some() {
            wrapper = wrapper.eq(AsiGroup::group_code(), arg.group_code.clone().unwrap());
        }
        wrapper
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut AsiGroupColumn) {
        data.id = common.id;
    }
}
/**-------------------------------------------------------------------------------------------------
*struct:AsiGroupValuesService
*desc:列定义值保存
*author:String
*email:348040933@qq.com
*/
pub struct AsiGroupValuesService {}

impl Default for AsiGroupValuesService {
    fn default() -> Self {
        AsiGroupValuesService {}
    }
}

impl AsiGroupValuesService {
    pub async fn save_batch_values(&self, values: Vec<AsiGroupValuesDTO>) {
        let mut entitys = vec![];
        for value in values {
            let  e: AsiGroupValues = value.into();
            entitys.push(e);
        }
        self.save_batch(&mut entitys).await;
    }
}

impl CrudService<AsiGroupValues, AsiGroupValuesDTO, AsiQuery> for AsiGroupValuesService {
    fn get_wrapper(arg: &AsiQuery) -> Wrapper {
        RB.new_wrapper()
    }

    fn set_save_common_fields(&self, common: CommonField, data: &mut AsiGroupValues) {
        data.id = common.id;
    }
}
