use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::entity::sys_config_tab_entity::SysConfigTab;
#[derive(
  Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default,
)]
#[getset(get = "pub", set = "pub")]
pub struct SysConfigTabDTO {
  id: Option<i32>,           // 配置分类id
  pid: Option<i32>,          // 上级分类id
  title: Option<String>,     // 配置分类名称
  eng_title: Option<String>, // 配置分类英文名称
  status: Option<i32>,       // 配置分类状态
  info: Option<i32>,         // 配置分类是否显示
  icon: Option<String>,      // 图标
  config_type: Option<i32>,  // 配置类型
  sort: Option<i32>,         // 排序
}

impl Into<SysConfigTab> for SysConfigTabDTO {
  fn into(self) -> SysConfigTab {
    SysConfigTab {
      id: self.id,
      pid: self.pid,
      title: self.title,
      eng_title: self.eng_title,
      status: self.status,
      info: self.info,
      icon: self.icon,
      config_type: self.config_type,
      sort: self.sort,
    }
  }
}

impl From<SysConfigTab> for SysConfigTabDTO {
  fn from(arg: SysConfigTab) -> Self {
    Self {
      id: arg.id,
      pid: arg.pid,
      title: arg.title,
      eng_title: arg.eng_title,
      status: arg.status,
      info: arg.info,
      icon: arg.icon,
      config_type: arg.config_type,
      sort: arg.sort,
    }
  }
}
