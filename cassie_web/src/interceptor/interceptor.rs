use super::core::build_delete;
use super::core::build_insert;
use super::core::build_select;
use super::core::build_update;
use super::core::has_table;
use super::core::intercept_delete;
use super::core::intercept_query;
use super::core::intercept_update;
use crate::APPLICATION_CONTEXT;
use cached::proc_macro::cached;
use cassie_config::config::ApplicationConfig;
use cassie_domain::request::RequestModel;
use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;
use sqlparser::ast::Statement::{Delete, Insert, Query as Iquwey, Update};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
//租户化拦截器 租户化核心实现
#[derive(Debug)]
pub struct AgencyInterceptor {
    //是否开始租户
    pub enable: bool,
    //租户的字段
    pub column: String,
    //需要忽略的表
    pub ignore_table: Vec<String>,
}
impl AgencyInterceptor {}
impl SqlIntercept for AgencyInterceptor {
    //sql拦截逻辑
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        //判断是否开启租户化
        if self.enable {
            if let Some(request_model) = APPLICATION_CONTEXT.try_get_local::<RequestModel>() {
                println!("{:?}", request_model);
                *sql = build(sql.clone(), request_model.agency_code.clone());
            }
        }
        return Ok(());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

//租户化sql生成
#[cached(time = 3600, size = 100)]
pub fn build(up_sql: String, agency_code: String) -> String {
    //获取配置类
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //获取默认数据库方言
    let dialect = GenericDialect {};
    //解析sql
    let atc = Parser::parse_sql(&dialect, &up_sql).unwrap();
    //这里只有一条sql所以获取第一条  SQL可以解析多个
    if let Some(data) = atc.get(0) {
        //模式匹配 只管 select 的情况
        match data {
            Iquwey(q) => {
                if let sqlparser::ast::SetExpr::Select(select) = &q.body {
                    if intercept_query(*select.clone()) {
                        return build_select(*select.clone(), agency_code);
                    }
                }
            }
            Insert {
                or,
                table_name,
                columns,
                overwrite,
                source,
                partitioned,
                after_columns,
                table,
                on,
            } => {
                //判断是否需要租户化
                if !has_table(table_name.clone(), config) {
                    match build_insert(
                        agency_code,
                        or.clone(),
                        table_name.clone(),
                        columns.clone(),
                        overwrite.clone(),
                        source.clone(),
                        partitioned.clone(),
                        after_columns.clone(),
                        table.clone(),
                        on.clone(),
                    ) {
                        Some(sql) => {
                            return sql;
                        }
                        None => println!(),
                    }
                }
            }
            Update {
                table,
                assignments,
                selection,
            } => if intercept_update(table.clone(), selection.clone()) {
                return build_update(agency_code, table.clone(), assignments.clone(),selection.clone());
            },
            Delete {
                table_name,
                selection,
            } => {
                if intercept_delete(table_name.clone(), selection.clone()) {
                    return build_delete(agency_code, table_name.clone(), selection.clone());
                }
            }
            _ => {}
        }
    }
    up_sql
}
