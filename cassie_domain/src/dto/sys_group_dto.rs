use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use crate::entity::sys_group_entity::SysGroup;
#[derive(
  Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default,
)]
#[getset(get = "pub", set = "pub")]
pub struct SysGroupDTO {
  id: Option<i32>,             // 组合数据ID
  cate_id: Option<i32>,        // 分类id
  name: Option<String>,        // 数据组名称
  info: Option<String>,        // 数据提示
  config_name: Option<String>, // 数据字段
  fields: Option<String>,      // 数据组字段以及类型（json数据）
}

impl Into<SysGroup> for SysGroupDTO {
  fn into(self) -> SysGroup {
    SysGroup {
      id: self.id,
      cate_id: self.cate_id,
      name: self.name,
      info: self.info,
      config_name: self.config_name,
      fields: self.fields,
    }
  }
}

impl From<SysGroup> for SysGroupDTO {
  fn from(arg: SysGroup) -> Self {
    Self {
      id: arg.id,
      cate_id: arg.cate_id,
      name: arg.name,
      info: arg.info,
      config_name: arg.config_name,
      fields: arg.fields,
    }
  }
}
