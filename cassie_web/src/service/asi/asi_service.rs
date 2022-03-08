
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use crate::{AsiQuery, REQUEST_CONTEXT, CONTEXT, RB};
use crate::dto::asi_dto::{AsiGroupColumnDTO, AsiGroupDTO, AsiGroupValuesDTO};
use crate::entity::asi_entitys::{AsiGroup, AsiGroupColumn, AsiGroupValues};
use crate::entity::sys_entitys::CommonField;
use crate::service::crud_service::CrudService;
use rbatis::plugin::snowflake::new_snowflake_id;
use cassie_common::error::Result;

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
        let count = crate::RB.fetch_count_by_wrapper::<AsiGroup>(
            RB.new_wrapper().eq(AsiGroup::group_code(), g.unwrap())).await;
        if let Ok(c) = count {
            return Err(cassie_common::error::Error::from("group_code已经存在".to_string()));
        }
        let tls = REQUEST_CONTEXT.clone();
        let a = tls.get().unwrap();
        let mut entity: AsiGroup = group.into();
        entity.agency_code = Some(a.agency_code.clone());
        self
            .save(&mut entity)
            .await
    }
}


impl Default for AsiGroupService {
    fn default() -> Self {
        AsiGroupService { asi_column: AsiGroupColumnService::default(), asi_values: AsiGroupValuesService::default() }
    }
}

impl CrudService<AsiGroup, AsiGroupDTO, AsiQuery> for AsiGroupService {
    fn get_wrapper(arg: &AsiQuery) -> Wrapper {
        RB.new_wrapper()
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
        self.del_by_column(AsiGroupColumn::group_code(), group_code.clone().unwrap().as_str()).await;
        /*获取当前登录信息*/
        let tls = REQUEST_CONTEXT.clone();
        let a = tls.get().unwrap();

        let mut entitys = vec![];
        /*构造信息*/
        for column in columns {
            let mut e: AsiGroupColumn = column.into();
            e.id = Some(new_snowflake_id());
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
        RB.new_wrapper()
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
            let mut e: AsiGroupValues = value.into();
            let id = new_snowflake_id();
            e.id = Some(id);
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