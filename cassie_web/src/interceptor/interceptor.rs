use std::collections::HashMap;

use crate::{sql_intercept_map, APPLICATION_CONTEXT};

use async_std::sync::Mutex;
use cassie_domain::request::RequestModel;
use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;

//租户化拦截器
#[derive(Debug)]
pub struct AgencyInterceptor {
    //是否开始租户
    pub enable: bool,
    //租户的字段
    pub column: String,
    //需要忽略的表
    pub ignore_table: Vec<String>,
}
impl AgencyInterceptor {
    fn find_index(&self, sql: &String, ketword: &str) -> Option<usize> {
        match ketword {
            "WHERE" => match sql.find(ketword) {
                None => {
                    return None;
                }
                Some(i) => {
                    return Some(i + 5);
                }
            },
            "ORDER" | "GROUP" | "LIMIT" => sql.find(ketword),
            _ => {
                return Some(sql.len());
            }
        }
    }
    fn build(&self, up_sql: &String, keyword: Vec<String>) -> (usize, String) {
        let request_model = APPLICATION_CONTEXT.get_local::<RequestModel>();
        for key in keyword {
            match self.find_index(&up_sql, &key) {
                None => {}
                Some(size) => {
                    if key.eq("WHERE") {
                        return (
                            size,
                            format!(
                                "  {} = '{}' and ",
                                self.column.clone(),
                                request_model.agency_code.clone()
                            ),
                        );
                    }
                    return (
                        size,
                        format!(
                            " where {} = '{}' ",
                            self.column.clone(),
                            request_model.agency_code.clone()
                        ),
                    );
                }
            };
        }
        (0, String::new())
    }
    //拦截判断
    fn intercept(&self, sql: &String) -> bool {
        let s = sql.clone().to_uppercase();
        if !s.starts_with("SELECT") {
            return false;
        }
        //当前忽略的表中如果包含了当前表直接返回
        for table in self.ignore_table.iter() {
            if s.contains(&table.clone().to_uppercase()) {
                return false;
            }
        }
        let column_index = s.find(&self.column.clone().to_uppercase());
        let from_index = s.find(&"FROM".to_string());
        if let Some(index) = column_index {
            if index > from_index.unwrap() {
                return false;
            }
        }
        true
    }
}
impl SqlIntercept for AgencyInterceptor {
    //sql拦截逻辑
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        if self.enable && self.intercept(sql) {
            let request_model = APPLICATION_CONTEXT.get_local::<RequestModel>();
            let back_sql = sql.clone();
            let mut map = sql_intercept_map.get().lock().unwrap();
            if map.contains_key(&back_sql) {
                let (index, agency_code, sql_c) = map.get(&sql.clone()).unwrap();
                println!("命中缓存了1:{}", index);
                if request_model.agency_code.eq(agency_code) {
                    println!("命中缓存了");
                    sql.insert_str(index.clone(), &sql_c);
                }
                return Ok(());
            }
            let up_sql = sql.clone().to_uppercase();
            //修改租户化方式直接拼接 不可更该顺序
            let keywords = vec![
                "WHERE".to_string(),
                "GROUP".to_string(),
                "ORDER".to_string(),
                "LIMIT".to_string(),
                "".to_string(),
            ];

            let (index, sql_c) = self.build(&up_sql, keywords);
            sql.insert_str(index, &sql_c);
            println!("拆入缓存:{}", index);
            map.insert(back_sql, (index, request_model.agency_code.clone(), sql_c));
        }
        return Ok(());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
