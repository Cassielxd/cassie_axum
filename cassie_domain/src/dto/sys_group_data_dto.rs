use crate::entity::sys_group_data_entity::SysGroupData;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;
#[derive(
  Serialize, Deserialize, Validate, Clone, Debug, Getters, Setters, Default,
)]
#[getset(get = "pub", set = "pub")]
pub struct SysGroupDataDTO {
  id: Option<i32>,       // 组合数据详情ID
  gid: Option<i32>,      // 对应的数据组id
  value: Option<String>, // 数据组对应的数据值（json数据）
  add_time: Option<i32>, // 添加数据时间
  sort: Option<i32>,     // 数据排序
  status: Option<i32>,   // 状态（1：开启；2：关闭；）
}

impl Into<SysGroupData> for SysGroupDataDTO {
  fn into(self) -> SysGroupData {
    SysGroupData {
      id: self.id,
      gid: self.gid,
      value: self.value,
      add_time: self.add_time,
      sort: self.sort,
      status: self.status,
    }
  }
}

impl From<SysGroupData> for SysGroupDataDTO {
  fn from(arg: SysGroupData) -> Self {
    Self {
      id: arg.id,
      gid: arg.gid,
      value: arg.value,
      add_time: arg.add_time,
      sort: arg.sort,
      status: arg.status,
    }
  }
}
