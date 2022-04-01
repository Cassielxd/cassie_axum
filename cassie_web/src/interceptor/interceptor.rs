use crate::APPLICATION_CONTEXT;
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
    //拦截判断
    fn intercept(&self, sql: &String) -> bool {
        let s = sql.clone().to_uppercase();
        //当前sql已经包含column 直接返回
        if s.contains(&self.column.clone().to_uppercase()) {
            return false;
        }
        //当前忽略的表中如果包含了当前表直接返回
        for table in self.ignore_table.iter() {
            if s.contains(&table.clone().to_uppercase()) {
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
            //修改租户化方式直接拼接 不使用占位符
            let w = if !sql.clone().to_uppercase().contains("WHERE") {
                format!(
                    " where  {} = '{}'",
                    self.column.clone(),
                    request_model.agency_code.clone()
                )
            } else {
                format!(
                    " and {} = '{}'",
                    self.column.clone(),
                    request_model.agency_code.clone()
                )
            };
            sql.insert_str(sql.len(), &w);
        }
        return Ok(());
    }
}
