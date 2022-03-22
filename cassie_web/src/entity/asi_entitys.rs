use crate::utils::tree::TreeModel;

/**
*struct:CiciSystemGroup
*desc:动态表单定义
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:asi_group)]
#[derive(Clone, Debug)]
pub struct AsiGroup {
    pub id: Option<i64>,
    pub cate_id: Option<i64>,
    pub name: Option<String>,
    pub info: Option<String>,
    pub group_code: Option<String>,
    pub agency_code: Option<String>,
    pub group_type: Option<String>,
    pub parent_group_code: Option<String>,
}
impl TreeModel for  AsiGroup{
    fn get_pid(&self) -> Option<String> {
        Some(self.parent_group_code.clone().unwrap())
    }

    fn get_id(&self) -> Option<String> {
        Some(self.group_code.clone().unwrap())
    }
}

impl_field_name_method!(AsiGroup {
    id,
    cate_id,
    name,
    info,
    group_code,
    agency_code,
    parent_group_code
});

/**
*struct:CiciSystemGroupColumn
*desc:动态表单Column定义外键关联 CiciSystemGroup
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:asi_group_column)]
#[derive(Clone, Debug)]
pub struct AsiGroupColumn{
    pub id: Option<i64>,
    pub agency_code: Option<String>,
    pub product_code: Option<String>,
    pub group_code: Option<String>,
    pub column_code: Option<String>,
    pub column_name: Option<String>,
    pub data_type: Option<String>,
    pub example_value: Option<String>,
    pub max_length: Option<i64>,
    pub is_required: Option<String>,
    pub display_order: Option<i8>,
    pub default_value: Option<String>,
    pub display_length: Option<i64>,
    pub is_display: Option<i8>,
}
impl_field_name_method!(AsiGroupColumn {
    id,
    agency_code,
    product_code,
    group_code,
    column_code,
    column_name,
    data_type,
    example_value,
    max_length,
    is_required,
    display_order,
    default_value,
    display_length,
    is_display
});

/**
 *struct:CiciSystemGroupValues
 *desc:动态表单值存储表 外键映射CiciSystemGroupColumn
       定义表 和 业务表ref_id 是对应的业务主键
 *author:String
 *email:348040933@qq.com
 */
#[crud_table(table_name:asi_group_values)]
#[derive(Clone, Debug)]
pub struct AsiGroupValues{
    pub id: Option<i64>,
    pub agency_code: Option<String>,
    pub product_code: Option<String>,
    pub group_code: Option<String>,
    pub column_code: Option<String>,
    pub column_value: Option<String>,
    pub ref_id: Option<i64>,
}
impl_field_name_method!(AsiGroupValues {
    id,
    agency_code,
    product_code,
    group_code,
    column_code,
    column_value,
    ref_id
});