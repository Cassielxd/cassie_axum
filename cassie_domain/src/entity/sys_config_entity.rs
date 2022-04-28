#[crud_table(table_name:sys_config)]
#[derive(Clone, Debug)]
pub struct SysConfig {
  pub id: Option<i32>,            // 配置id
  pub menu_name: Option<String>,  // 字段名称
  pub input_type: Option<String>, // 类型(文本框,单选按钮...)
  pub from_type: Option<String>,  // 表单类型
  pub config_tab_id: Option<i32>, // 配置分类id
  pub parameter: Option<String>,  // 规则 单选框和多选框
  pub upload_type: Option<i32>,   // 上传文件格式1单图2多图3文件
  pub required: Option<String>,   // 规则
  pub width: Option<i32>,         // 多行文本框的宽度
  pub high: Option<i32>,          // 多行文框的高度
  pub value: Option<String>,      // 默认值
  pub info: Option<String>,       // 配置名称
  pub desc: Option<String>,       // 配置简介
  pub sort: Option<i32>,          // 排序
  pub status: Option<i32>,        // 是否隐藏
}
impl_field_name_method!(SysConfig {
  id,
  menu_name,
  input_type,
  from_type,
  config_tab_id,
  parameter,
  upload_type,
  required,
  width,
  high,
  value,
  info,
  desc,
  sort,
  status
});
