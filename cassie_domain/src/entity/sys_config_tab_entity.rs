#[crud_table(table_name:sys_config_tab)]
#[derive(Clone, Debug)]
pub struct SysConfigTab {
  pub id: Option<i32>,           // 配置分类id
  pub pid: Option<i32>,          // 上级分类id
  pub title: Option<String>,     // 配置分类名称
  pub eng_title: Option<String>, // 配置分类英文名称
  pub status: Option<i32>,       // 配置分类状态
  pub info: Option<i32>,         // 配置分类是否显示
  pub icon: Option<String>,      // 图标
  pub config_type: Option<i32>,  // 配置类型
  pub sort: Option<i32>,         // 排序
}
impl_field_name_method!(SysConfigTab {
  id,
  pid,
  title,
  eng_title,
  status,
  info,
  icon,
  config_type,
  sort,
});
