#[crud_table(table_name:sys_group_data)]
#[derive(Clone, Debug)]
pub struct SysGroupData {
    pub id: Option<i32>,       // 组合数据详情ID
    pub gid: Option<i32>,      // 对应的数据组id
    pub value: Option<String>, // 数据组对应的数据值（json数据）
    pub add_time: Option<i32>, // 添加数据时间
    pub sort: Option<i32>,     // 数据排序
    pub status: Option<i32>,   // 状态（1：开启；2：关闭；）
}
impl_field_name_method!(SysGroupData {
    id,
    gid,
    value,
    add_time,
    sort,
    status,
});
