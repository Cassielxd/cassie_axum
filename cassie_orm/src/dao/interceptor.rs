use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;


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
    fn intercept(&self, sql: &String) -> bool {
        let s = sql.clone().to_uppercase();
        for row in self.ignore_table.iter() {
            if s.contains(&row.clone().to_uppercase()) {
                return false;
            }
        }
        true
    }
}
impl SqlIntercept for AgencyInterceptor {
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        if self.enable && self.intercept(sql) {}
        println!("拦截器sql:{}", sql.clone());
        return Ok(());
    }
}