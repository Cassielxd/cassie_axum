use rbatis::DateTimeNative;

#[crud_table(table_name:sys_log_login)]
#[derive(Clone, Debug)]
pub struct SysLogLogin {
    pub id: Option<i64>,
    pub operation: Option<i8>,
    pub user_agent: Option<String>,
    pub ip: Option<String>,
    pub creator_name: Option<String>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysLogLogin {
    id,
    operation,
    user_agent,
    ip,
    creator_name,
    creator,
    create_date
});

#[crud_table(table_name:sys_log_operation)]
#[derive(Clone, Debug)]
pub struct SysLogOperation {
    pub id: Option<i64>,
    pub operation: Option<i8>,
    pub request_uri: Option<String>,
    pub ip: Option<String>,
    pub creator_name: Option<String>,
    pub request_params: Option<String>,
    pub request_method: Option<String>,
    pub request_time: Option<String>,
    pub status: Option<i8>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysLogOperation {
    id,
    operation,
    request_uri,
    request_params,
    request_method,
    request_time,
    status,
    ip,
    creator_name,
    creator,
    create_date
});
