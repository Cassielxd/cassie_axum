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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysDictTypeDTO {
    pub id: Option<i64>,
    pub dict_type: Option<String>,
    pub dict_name: Option<String>,
    pub remark: Option<String>,
    pub sort: Option<u32>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysDictTypeDTO {
    id,
    dict_type,
    dict_name,
    sort,
    remark,
    creator,
    create_date,
    updater,
    update_date,
});

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
        }
    }
}

/**
 * 字典值DTO
 */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysDictDataDTO {
    pub id: Option<i64>,
    pub dict_type_id: Option<i64>,
    pub dict_label: Option<String>,
    pub dict_value: Option<String>,
    pub remark: Option<String>,
    pub sort: Option<u32>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysDictDataDTO {
    id,
    dict_type_id,
    dict_label,
    dict_value,
    sort,
    remark,
    creator,
    create_date,
    updater,
    update_date,
});

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
