use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;

use crate::CASSIE_CONFIG;

#[derive(Debug)]
pub struct AgencyInterceptor {}

impl SqlIntercept for AgencyInterceptor {
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        println!("sql:{}", sql.clone());
        println!("args:{:?}", args.clone());
        if CASSIE_CONFIG.tenant.enable {
            for table in &CASSIE_CONFIG.tenant.ignore_table {}
        }
        return Ok(());
    }
}
