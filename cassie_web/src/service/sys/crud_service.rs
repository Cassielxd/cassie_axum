use crate::entity::pagedata::PageData;
use crate::entity::sys_entitys::CommonField;
use crate::{CONTEXT, RB, REQUEST_CONTEXT};
use async_trait::async_trait;
use cassie_common::error::Result;
use rbatis::crud::{CRUDTable, Skip, CRUD};
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::new_snowflake_id;
use rbatis::wrapper::Wrapper;
use rbatis::DateTimeNative;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::From;
/**
 *struct:CrudService
 *desc:orm基础CRUD实现
 *author:String
 *email:348040933@qq.com
 */
#[async_trait]
pub trait CrudService<Entity, Dto, Params>: Sync + Send
where
    Entity: CRUDTable + DeserializeOwned,
    Dto: From<Entity> + Send + Sync + Serialize,
    Params: Send + Sync + Serialize,
{
    /**
     * 获取查询条件Wrapper
     * 子类实现
     */
    fn get_wrapper(arg: &Params) -> Wrapper;
    /**设置公共的字段保存方法*/
    fn set_save_common_fields(&self, common: CommonField, data: &mut Entity);

    /**
     * 公共分页查询方法
     */
    async fn page(&self, arg: &Params, page: PageData) -> Result<Page<Dto>> {
        //构建查询条件
        let wrapper = Self::get_wrapper(arg);
        //构建分页条件
        let page_request =
            PageRequest::new(page.page_no.unwrap_or(1), page.page_size.unwrap_or(10));
        //执行分页查询
        let data_page: Page<Entity> = RB.fetch_page_by_wrapper(wrapper, &page_request).await?;
        //将Entity实体转换成 Vo对象返回
        let mut vos = vec![];
        for x in data_page.records {
            vos.push(Dto::from(x));
        }
        Ok(Page::<Dto> {
            records: vos,
            total: data_page.total,
            pages: data_page.pages,
            page_no: data_page.page_no,
            page_size: data_page.page_size,
            search_count: data_page.search_count,
        })
    }
    /**
     * 公共列表查询方法
     */
    async fn list(&self, arg: &Params) -> Result<Vec<Dto>> {
        //构建查询条件
        let wrapper = Self::get_wrapper(arg);
        //执行查询
        let list: Vec<Entity> = RB.fetch_list_by_wrapper(wrapper).await?;
        let mut vos = vec![];
        //将Entity实体转换成 Vo对象 返回
        for x in list {
            vos.push(Dto::from(x));
        }
        Ok(vos)
    }

    
    /**
     * 根据id更新实体
     */
    async fn update_by_id(&self, id: String, mut data: &Entity) {
        let wrapper = RB.new_wrapper().eq("id", id);
        RB.update_by_wrapper(
            &mut data,
            wrapper,
            &[Skip::Column("id"), Skip::Column("create_date")],
        )
        .await;
    }
    /**
     * 根据id查询条件查询单个值
     */
    async fn get(&self, id: String) -> Result<Dto> {
        let wrapper = RB.new_wrapper().eq("id", id);
        let detail: Entity = RB.fetch_by_wrapper(wrapper).await?;
        let vo = Dto::from(detail);
        return Ok(vo);
    }
    /**
     * 保存实体
     */
    async fn save(&self, data: &mut Entity) -> Result<i64> {
        /*设置创建人*/

        let tls = REQUEST_CONTEXT.clone();
        let creator = if let Some(a) = tls.get() { a.uid } else { 0 };
        /*设置公共字段*/
        self.set_save_common_fields(
            CommonField {
                id: Some(0),
                creator: Some(creator),
                create_date: Some(DateTimeNative::now()),
                updater: None,
                update_date: None,
            },
            data,
        );
        let result = RB.save(data, &[Skip::Column("id")]).await?;
        return Ok(result.last_insert_id.unwrap());
    }
    /**
     * 批量保存实体
     */
    async fn save_batch(&self, mut list: &Vec<Entity>) {
        RB.save_batch(&mut list, &[Skip::Column("id")]).await;
    }
    /**
     * 删除实体 逻辑删除
     */
    async fn del(&self, id: &String) {
        RB.remove_by_column::<Entity, _>("id", id).await;
    }
    /**
     * 根据字段实体
     */
    async fn del_by_column(&self, column: &str, column_value: &str) {
        RB.remove_by_column::<Entity, _>(column, column_value).await;
    }
    /**
     * 批量删除实体 逻辑删除
     */
    async fn del_batch(&self, ids: &Vec<u64>) {
        RB.remove_batch_by_column::<Entity, _>("id", ids).await;
    }
}
