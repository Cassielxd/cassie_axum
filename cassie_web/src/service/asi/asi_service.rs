use crate::dto::asi_dto::{AsiGroupColumnDTO, AsiGroupDTO, AsiGroupValuesDTO};
use crate::entity::asi_entitys::{AsiGroup, AsiGroupColumn, AsiGroupValues};
use crate::entity::sys_entitys::CommonField;
use crate::service::crud_service::CrudService;
use crate::{AsiQuery, MDB, RB, REQUEST_CONTEXT};
use cassie_common::error::Result;
use futures::{TryFutureExt, TryStreamExt};
use std::collections::HashMap;

use cassie_common::error::Error;
use mongodb::bson::{bson, doc, Bson, Document, Uuid};
use mongodb::options::FindOptions;
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;

use super::asi_validation::{validate_value, validate_values};

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
    /**
     *method:save_values_for_from
     *desc:保存列下面对应的值 from
     *author:String
     *email:348040933@qq.com
     */
    pub async fn save_values_for_from(
        &self,
        id: String,
        args: HashMap<String, HashMap<String, String>>,
    ) -> Result<bool> {
        for (key, value) in args {
            let query = AsiQuery {
                column_code: None,
                group_code: Option::Some(key.clone()),
                page: None,
                limit: None,
                order: None,
                order_field: None,
            };
            //获取分组定义信息
            let group = self.list(&query).await;
            match group {
                Ok(data) => {
                    let g = data.get(0).unwrap();
                    //获取定义列信息
                    let columns = self.asi_column.list(&query).await;
                    match columns {
                        Ok(cloums_list) => {
                            ///验证数据定义
                            match validate_value(&cloums_list, &value) {
                                Ok(_) => {
                                    //验证通过 保存数据
                                    self.save_from_values(&id, g, value).await;
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
        }

        return Ok(true);
    }

    /**
     *method:save_values_for_table
     *desc:保存列下面对应的值 table
     *author:String
     *email:348040933@qq.com
     */
    pub async fn save_values_for_table(
        &self,
        id: String,
        args: HashMap<String, Vec<HashMap<String, String>>>,
    ) -> Result<bool> {
        for (key, value) in args {
            let query = AsiQuery {
                column_code: None,
                group_code: Option::Some(key.clone()),
                page: None,
                limit: None,
                order: None,
                order_field: None,
            };
            //获取分组定义信息
            let group = self.list(&query).await;
            match group {
                Ok(data) => {
                    let g = data.get(0).unwrap();
                    //获取定义列信息
                    let columns = self.asi_column.list(&query).await;
                    match columns {
                        Ok(cloums_list) => {
                            ///验证数据定义
                            match validate_values(&cloums_list, &value) {
                                Ok(_) => {
                                    //验证通过 保存数据
                                    self.save_table_values(&id, g, value).await;
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
        }

        return Ok(true);
    }

    pub async fn save_from_values(
        &self,
        id: &String,
        group: &AsiGroupDTO,
        values: HashMap<String, String>,
    ) {
        let collection = MDB.collection::<Document>(build_table(group).as_str());

        let mut doc = Document::new();
        doc.insert("_id", id.clone());
        doc.insert("entity_id", id.clone());
        for (key, value) in values {
            doc.insert(key, value);
        }
        collection.insert_one(doc, None).await;
    }

    pub async fn save_table_values(
        &self,
        id: &String,
        group: &AsiGroupDTO,
        values_map: Vec<HashMap<String, String>>,
    ) {
        let collection = MDB.collection::<Document>(build_table(group).as_str());
        let mut docs = vec![];
        for values in values_map {
            let mut doc = Document::new();
            doc.insert("_id", Uuid::new().to_string());
            doc.insert("entity_id", id.clone());
            doc.insert(
                AsiGroupDTO::agency_code().to_string(),
                group.agency_code.clone().unwrap(),
            );
            doc.insert(
                AsiGroupDTO::group_code().to_string(),
                group.group_code.clone().unwrap(),
            );
            for (key, value) in values {
                doc.insert(key, value);
            }
            docs.push(doc);
        }
        collection.insert_many(docs, None).await;
    }
    ///查询values
    pub async fn value_list(
        &self,
        id: &String,
        group: &AsiGroupDTO,
    ) -> Result<Vec<HashMap<String, Bson>>> {
        let query = AsiQuery {
            column_code: None,
            group_code: group.group_code.clone(),
            page: None,
            limit: None,
            order: None,
            order_field: None,
        };
        let columns = self.asi_column.list(&query).await.unwrap();

        let collection = MDB.collection::<Document>(build_table(group).as_str());
        let filter = doc! { "entity_id": id.clone() };
        let mut result = collection.find(filter, None).await.unwrap();
        let mut r = Vec::new();
        while let Ok(a) = result.try_next().await {
            match a {
                None => {
                    break;
                }
                Some(doc) => {
                    let mut d = HashMap::new();
                    //使用已经定义的列进行获取
                    for c in &columns {
                        if doc.contains_key(c.column_code.clone().unwrap()) {
                            d.insert(
                                c.column_code.clone().unwrap(),
                                doc.get(c.column_code.clone().unwrap()).unwrap().clone(),
                            );
                        } else {
                            d.insert(c.column_code.clone().unwrap(), Bson::String("".to_string()));
                        }
                    }
                    d.insert(
                        AsiGroupDTO::agency_code().to_string(),
                        doc.get(AsiGroupDTO::agency_code().to_string())
                            .unwrap()
                            .clone(),
                    );
                    d.insert(
                        AsiGroupDTO::group_code().to_string(),
                        doc.get(AsiGroupDTO::group_code().to_string())
                            .unwrap()
                            .clone(),
                    );
                    r.push(d);
                }
            }
        }
        Ok(r)
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

impl AsiGroupValuesService {}

fn build_table(group: &AsiGroupDTO) -> String {
    format!(
        "{}-{}-{}-{}",
        group.agency_code.clone().unwrap(),
        group.group_type.clone().unwrap(),
        group.parent_group_code.clone().unwrap(),
        group.group_code.clone().unwrap()
    )
}
