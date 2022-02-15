use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbson::Bson;
use rbatis::Error;

#[derive(Debug)]
pub struct AgencyInterceptor{}

impl SqlIntercept for AgencyInterceptor{
    fn do_intercept(&self, rb: &Rbatis, sql: &mut String, args: &mut Vec<Bson>, is_prepared_sql: bool) -> Result<(), Error> {
        return Ok(());
    }
}