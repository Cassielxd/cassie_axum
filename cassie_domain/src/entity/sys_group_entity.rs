#[crud_table(table_name:sys_group)]
#[derive(Clone, Debug)]
pub struct SysGroup {
    pub id: Option<i32>,             // 组合数据ID
    pub cate_id: Option<i32>,        // 分类id
    pub name: Option<String>,        // 数据组名称
    pub info: Option<String>,        // 数据提示
    pub config_name: Option<String>, // 数据字段
    pub fields: Option<String>,      // 数据组字段以及类型（json数据）
}
impl_field_name_method!(SysGroup {
    id,
    cate_id,
    name,
    info,
    config_name,
    fields,
});
