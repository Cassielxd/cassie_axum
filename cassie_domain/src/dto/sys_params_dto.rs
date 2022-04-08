use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

use crate::entity::sys_entitys::SysParams;

/**
 * 系统参数定义DTO
 */
#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysParamsDTO {
    id: Option<i64>,
    param_code: Option<String>,
    param_value: Option<String>,
    remark: Option<String>,
    del_flag: Option<u8>,
    creator: Option<i64>,
    create_date: Option<DateTimeNative>,
    updater: Option<i64>,
    update_date: Option<DateTimeNative>,
}

impl Into<SysParams> for SysParamsDTO {
    fn into(self) -> SysParams {
        SysParams {
            id: self.id,
            remark: self.remark,
            creator: self.creator,
            create_date: self.create_date,
            updater: self.updater,
            update_date: self.update_date,
            param_code: self.param_code,
            param_value: self.param_value,
            del_flag: self.del_flag,
        }
    }
}

impl From<SysParams> for SysParamsDTO {
    fn from(arg: SysParams) -> Self {
        Self {
            id: arg.id,
            remark: arg.remark,
            creator: arg.creator,
            create_date: arg.create_date,
            updater: arg.updater,
            update_date: arg.update_date,
            param_code: arg.param_code,
            param_value: arg.param_value,

            del_flag: arg.del_flag,
        }
    }
}
