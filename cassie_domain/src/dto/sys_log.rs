use crate::entity::log::{SysLogLogin, SysLogOperation};
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysLogLoginDto {
    pub id: Option<i64>,
    pub operation: Option<i8>,
    pub user_agent: Option<String>,
    pub ip: Option<String>,
    pub creator_name: Option<String>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}

impl From<SysLogLogin> for SysLogLoginDto {
    fn from(arg: SysLogLogin) -> Self {
        Self {
            id: arg.id,
            operation: arg.operation,
            user_agent: arg.user_agent,
            ip: arg.ip,
            creator_name: arg.creator_name,
            creator: arg.creator,
            create_date: arg.create_date,
        }
    }
}

impl Into<SysLogLogin> for SysLogLoginDto {
    fn into(self) -> SysLogLogin {
        SysLogLogin {
            id: self.id,
            operation: self.operation,
            user_agent: self.user_agent,
            ip: self.ip,
            creator_name: self.creator_name,
            creator: self.creator,
            create_date: self.create_date,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysLogOperationDto {
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
impl Into<SysLogOperation> for SysLogOperationDto {
    fn into(self) -> SysLogOperation {
        SysLogOperation {
            id: self.id,
            operation: self.operation,
            request_uri: self.request_uri,
            ip: self.ip,
            creator_name: self.creator_name,
            request_params: self.request_params,
            request_method: self.request_method,
            request_time: self.request_time,
            status: self.status,
            creator: self.creator,
            create_date: self.create_date,
        }
    }
}
impl From<SysLogOperation> for SysLogOperationDto {
    fn from(arg: SysLogOperation) -> Self {
        Self {
            id: arg.id,
            operation: arg.operation,
            request_uri: arg.request_uri,
            ip: arg.ip,
            creator_name: arg.creator_name,
            request_params: arg.request_params,
            request_method: arg.request_method,
            request_time: arg.request_time,
            status: arg.status,
            creator: arg.creator,
            create_date: arg.create_date,
        }
    }
}
