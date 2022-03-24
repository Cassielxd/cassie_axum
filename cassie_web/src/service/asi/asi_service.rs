use crate::dto::asi_dto::{AsiGroupColumnDTO, AsiGroupDTO};
use crate::entity::asi_entitys::{AsiGroup, AsiGroupColumn};
use crate::entity::sys_entitys::CommonField;
use crate::service::crud_service::CrudService;
use crate::utils::tree::TreeService;
use crate::{AsiQuery, MDB, RB, REQUEST_CONTEXT};
use cassie_common::error::Result;
use futures::TryStreamExt;
use mongodb::options::UpdateModifications;
use std::collections::HashMap;

use super::asi_validation::{validate_value, validate_values};
use crate::entity::PageData;
use cassie_common::error::Error;
use cassie_common::utils::string::IsEmpty;
use mongodb::bson::{doc, Bson, Document, Uuid};
use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use rbatis::{Page, PageRequest};

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
// -> Result<Page<AsiGroupDTO>>
impl TreeService<AsiGroup, AsiGroupDTO> for AsiGroupService {
    fn set_children(&self, arg: &mut AsiGroupDTO, childs: Option<Vec<AsiGroupDTO>>) {
        arg.children = childs;
    }
}

impl AsiGroupService {
    /**
     * @description:  get_group_lis 根据条件获取所有group 并生成树
     * @param: null
     * @return:
     * @author String
     * @date: 2022/3/22 18:03
     */
    pub async fn get_group_list(&self, group: AsiQuery) -> Result<Vec<AsiGroupDTO>> {
        let wrapper = Self::get_wrapper(&group);
        let list: Vec<AsiGroup> = RB.fetch_list_by_wrapper(wrapper).await?;
        Ok(self.build(list))
    }
    /**
     * @description:  get_group_page根据条件获取group page 并生成tree
     * @param: null
     * @return:
     * @author String
     * @date: 2022/3/22 18:04
     */
    pub async fn get_group_page(&self, group: AsiQuery) -> Result<Page<AsiGroupDTO>> {
        let wrapper = Self::get_wrapper(&group);
        //构建分页条件
        let page_request = PageRequest::new(group.page.unwrap_or(1), group.limit.unwrap_or(10));
        //执行分页查询
        let data_page: Page<AsiGroup> = RB.fetch_page_by_wrapper(wrapper, &page_request).await?;
        let records = data_page.records;

        Ok(Page::<AsiGroupDTO> {
            records: self.build(records),
            total: data_page.total,
            pages: data_page.pages,
            page_no: data_page.page_no,
            page_size: data_page.page_size,
            search_count: data_page.search_count,
        })
    }
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
            let searchc = vec![key.clone()];
            //获取分组定义信息
            let group = self
                .fetch_list_by_column("group_code", &searchc)
                .await
                .map_err(|e| Error::E("业务分组定义不存在!".to_string()))
                .unwrap();
            let g = group.get(0).unwrap();
            //获取定义列信息
            let columns = self
                .asi_column
                .fetch_list_by_column("group_code", &searchc)
                .await
                .map_err(|e| Error::E("列定义不存在!".to_string()))
                .unwrap();
            //验证数据定义
            match validate_value(&columns, &value) {
                Ok(_) => {
                    //验证通过 保存数据
                    self.save_from_values(&id, g, value).await;
                }
                Err(e) => {
                    return Err(e);
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
            let searchc = vec![key.clone()];
            //获取分组定义信息
            let group = self
                .fetch_list_by_column("group_code", &searchc)
                .await
                .map_err(|e| Error::E("业务分组定义不存在!".to_string()))
                .unwrap();
            let g = group.get(0).unwrap();
            //获取定义列信息
            let columns = self
                .asi_column
                .fetch_list_by_column("group_code", &searchc)
                .await
                .map_err(|e| Error::E("列定义不存在!".to_string()))
                .unwrap();
            //验证数据定义
            match validate_values(&columns, &value) {
                Ok(_) => {
                    //验证通过 保存数据
                    self.save_table_values(&id, g, value).await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(true);
    }
    //构建插入语句
    fn build_data(
        &self,
        id: &String,
        group: &AsiGroupDTO,
        values: &HashMap<String, String>,
    ) -> (bool, Document) {
        let mut insert = false;
        let mut doc = Document::new();
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
        /*新增 如果values 已经有了_id会被覆盖 */
        if !doc.contains_key("_id") {
            insert = true;
            if group.group_type.eq(&Option::Some("FROM".to_string())) {
                doc.insert("_id", id.clone());
            } else {
                doc.insert("_id", Uuid::new().to_string());
            }
        }
        (insert, doc)
    }

    pub async fn save_from_values(
        &self,
        id: &String,
        group: &AsiGroupDTO,
        values: HashMap<String, String>,
    ) {
        let (insert, doc) = self.build_data(id, group, &values);
        let collection = MDB.collection::<Document>(build_table(group).as_str());
        if insert {
            collection.insert_one(doc, None).await;
        } else {
            let mut query = Document::new();
            query.insert("_id", doc.get("_id"));
            let mut update_doc = Document::new();
            update_doc.insert("$set", doc);
            collection
                .update_one(query, UpdateModifications::Document(update_doc), None)
                .await;
        }
    }

    pub async fn save_table_values(
        &self,
        id: &String,
        group: &AsiGroupDTO,
        values_map: Vec<HashMap<String, String>>,
    ) {
        let collection = MDB.collection::<Document>(build_table(group).as_str());
        let mut insert_docs = vec![];
        let mut update_docs = vec![];
        for values in values_map.iter() {
            let (insert, doc) = self.build_data(id, group, values);
            if insert {
                insert_docs.push(doc);
            } else {
                update_docs.push(doc);
            }
        }
        if !insert_docs.is_empty() {
            collection.insert_many(insert_docs, None).await;
        }
        if !update_docs.is_empty() {
            for i in update_docs {
                let mut doc = Document::new();
                doc.insert("_id", i.get("_id"));
                let mut update_doc = Document::new();
                update_doc.insert("$set", i);
                collection
                    .update_one(doc, UpdateModifications::Document(update_doc), None)
                    .await;
            }
        }
    }
    ///查询values
    pub async fn value_list(
        &self,
        id: &String,
        group: &AsiGroupDTO,
    ) -> Result<Vec<HashMap<String, Bson>>> {
        let columns = self
            .asi_column
            .fetch_list_by_column("group_code", &vec![group.group_code.clone().unwrap()])
            .await
            .unwrap();

        let collection = MDB.collection::<Document>(build_table(group).as_str());
        let filter = doc! { "entity_id": id.clone() };
        let mut result = collection.find(filter, None).await.unwrap();
        let mut r = Vec::new();
        while let Some(doc) = result
            .try_next()
            .await
            .map_err(|e| Error::E(e.to_string()))
            .unwrap()
        {
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
            d.insert(
                "_id".to_string(),
                doc.get("_id".to_string()).unwrap().clone(),
            );
            r.push(d);
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
        RB.new_wrapper()
            .do_if(!arg.group_code.is_empty(), |w| {
                w.eq(AsiGroup::group_code(), arg.group_code.clone().unwrap())
            })
            .do_if(!arg.parent_group_code.is_empty(), |w| {
                w.eq(
                    AsiGroup::parent_group_code(),
                    arg.parent_group_code.clone().unwrap(),
                )
            })
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
    pub async fn save_column(&self, group: AsiGroupDTO, column: AsiGroupColumnDTO) {
        let tls = REQUEST_CONTEXT.clone();
        let a = tls.get().unwrap();
        let id = column.id.clone();
        let mut entity: AsiGroupColumn = column.into();
        entity.group_code = group.group_code;
        entity.agency_code = Some(a.agency_code.clone());
        if let Some(i) = id {
            self.update_by_id(i.to_string(), &mut entity).await;
        } else {
            self.save(&mut entity).await;
        }
    }
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

        let mut entitys: Vec<AsiGroupColumn> = columns
            .iter()
            .map(|e| {
                let mut e: AsiGroupColumn = e.clone().into();
                e.group_code = group_code.clone();
                e.agency_code = Some(a.agency_code.clone());
                e
            })
            .collect();
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
        RB.new_wrapper().do_if(!arg.group_code.is_empty(), |w| {
            w.eq(AsiGroup::group_code(), arg.group_code.clone().unwrap())
        })
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
