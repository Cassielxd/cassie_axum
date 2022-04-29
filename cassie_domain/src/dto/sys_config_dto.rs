use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::entity::sys_config_entity::SysConfig;
#[derive(Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysConfigDTO {
    id: Option<i32>,            // 配置id
    menu_name: Option<String>,  // 字段名称
    input_type: Option<String>, // 类型(文本框,单选按钮...)
    from_type: Option<String>,  // 表单类型
    config_tab_id: Option<i32>, // 配置分类id
    parameter: Option<String>,  // 规则 单选框和多选框
    upload_type: Option<i32>,   // 上传文件格式1单图2多图3文件
    required: Option<String>,   // 规则
    width: Option<i32>,         // 多行文本框的宽度
    high: Option<i32>,          // 多行文框的高度
    value: Option<String>,      // 默认值
    info: Option<String>,       // 配置名称
    desc: Option<String>,       // 配置简介
    sort: Option<i32>,          // 排序
    status: Option<i32>,        // 是否隐藏
}

impl Into<SysConfig> for SysConfigDTO {
    fn into(self) -> SysConfig {
        SysConfig {
            id: self.id,
            menu_name: self.menu_name,
            input_type: self.input_type,
            from_type: self.from_type,
            config_tab_id: self.config_tab_id,
            parameter: self.parameter,
            upload_type: self.upload_type,
            required: self.required,
            width: self.width,
            high: self.high,
            value: self.value,
            info: self.info,
            desc: self.desc,
            sort: self.sort,
            status: self.status,
        }
    }
}

impl From<SysConfig> for SysConfigDTO {
    fn from(arg: SysConfig) -> Self {
        Self {
            id: arg.id,
            menu_name: arg.menu_name,
            input_type: arg.input_type,
            from_type: arg.from_type,
            config_tab_id: arg.config_tab_id,
            parameter: arg.parameter,
            upload_type: arg.upload_type,
            required: arg.required,
            width: arg.width,
            high: arg.high,
            value: arg.value,
            info: arg.info,
            desc: arg.desc,
            sort: arg.sort,
            status: arg.status,
        }
    }
}
