use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

use crate::entity::sys_entitys::{SysDictData, SysDictType};
/**
 * DTO 实现了From 和Into 用户 struct类型转换
 * author:String
 */

/**
 * 字典定义DTO
 */
#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysDictTypeDTO {
    id: Option<i64>,
    dict_type: Option<String>,
    dict_name: Option<String>,
    remark: Option<String>,
    sort: Option<u32>,
    creator: Option<i64>,
    create_date: Option<DateTimeNative>,
    updater: Option<i64>,
    update_date: Option<DateTimeNative>,
    data_list: Option<Vec<SysDictDataDTO>>,
}

impl Into<SysDictType> for SysDictTypeDTO {
    fn into(self) -> SysDictType {
        SysDictType {
            id: self.id,
            dict_type: self.dict_type,
            dict_name: self.dict_name,
            remark: self.remark,
            sort: self.sort,
            creator: self.creator,
            create_date: self.create_date,
            updater: self.updater,
            update_date: self.update_date,
        }
    }
}

impl From<SysDictType> for SysDictTypeDTO {
    fn from(arg: SysDictType) -> Self {
        Self {
            id: arg.id,
            dict_type: arg.dict_type,
            dict_name: arg.dict_name,
            remark: arg.remark,
            sort: arg.sort,
            creator: arg.creator,
            create_date: arg.create_date,
            updater: arg.updater,
            update_date: arg.update_date,
            data_list: Option::Some(Vec::<SysDictDataDTO>::new()),
        }
    }
}

/**
 * 字典值DTO
 */

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysDictDataDTO {
    id: Option<i64>,
    dict_type_id: Option<i64>,
    dict_label: Option<String>,
    dict_value: Option<String>,
    remark: Option<String>,
    sort: Option<u32>,
    creator: Option<i64>,
    create_date: Option<DateTimeNative>,
    updater: Option<i64>,
    update_date: Option<DateTimeNative>,
}

impl Into<SysDictData> for SysDictDataDTO {
    fn into(self) -> SysDictData {
        SysDictData {
            id: self.id,
            dict_type_id: self.dict_type_id,
            dict_label: self.dict_label,
            dict_value: self.dict_value,
            remark: self.remark,
            sort: self.sort,
            creator: self.creator,
            create_date: self.create_date,
            updater: self.updater,
            update_date: self.update_date,
        }
    }
}

impl From<SysDictData> for SysDictDataDTO {
    fn from(arg: SysDictData) -> Self {
        Self {
            id: arg.id,
            dict_type_id: arg.dict_type_id,
            dict_label: arg.dict_label,
            dict_value: arg.dict_value,
            remark: arg.remark,
            sort: arg.sort,
            creator: arg.creator,
            create_date: arg.create_date,
            updater: arg.updater,
            update_date: arg.update_date,
        }
    }
}
