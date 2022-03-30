use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;

#[derive(Debug)]
pub struct AgencyInterceptor {
    pub enable: bool,
    pub column: String,
    pub ignore_table: Vec<String>,
}
impl AgencyInterceptor {
    fn intercept(&self,sql:&String) -> bool {
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
        if self.enable {
            if self.intercept(sql) {
               todo!("待升级")
            }
        }
        println!("拦截器sql:{}",sql.clone());
        return Ok(());
    }
}
