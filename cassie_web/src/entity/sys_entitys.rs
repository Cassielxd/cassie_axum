use rbatis::DateTimeNative;

/**
*struct:SysUser
*desc:后台用户表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_user)]
#[derive(Clone, Debug)]
pub struct SysUser {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub real_name: Option<String>,
    pub head_url: Option<String>,
    pub gender: Option<u8>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub dept_id: Option<i32>,
    pub super_admin: Option<i32>,
    pub agency_code: Option<String>,
    pub remark: Option<String>,
    pub status: Option<i32>,
    pub del_flag: Option<i32>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}
/**
*struct:SysDictType
*desc:字典表 定义表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_dict_type)]
#[derive(Clone, Debug)]
pub struct SysDictType {
    pub id: Option<i64>,
    //字典类型
    pub dict_type: Option<String>,
    //字典名称
    pub dict_name: Option<String>,
    //备注
    pub remark: Option<String>,
    //排序
    pub sort: Option<u32>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}

impl_field_name_method!(SysUser {
    id,
    username,
    password,
    real_name,
    head_url,
    gender,
    email,
    mobile,
    dept_id,
    super_admin,
    remark,
    status,
    del_flag,
    creator,
    create_date,
    updater,
    update_date,
    agency_code
});
impl_field_name_method!(SysDictType {
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

/**
*struct:SysDictData
*desc:字典值表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_dict_data)]
#[derive(Clone, Debug)]
pub struct SysDictData {
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
impl_field_name_method!(SysDictData {
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

/**
*struct:SysParams
*desc:系统参数表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_params)]
#[derive(Clone, Debug)]
pub struct SysParams {
    pub id: Option<i64>,
    pub param_code: Option<String>,
    pub param_value: Option<String>,
    pub remark: Option<String>,
    pub del_flag: Option<u8>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysParams {
    id,
    param_code,
    param_value,
    del_flag,
    remark,
    creator,
    create_date,
    updater,
    update_date,
});

/**
*struct:SysRole
*desc:角色表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_role)]
#[derive(Clone, Debug)]
pub struct SysRole {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub remark: Option<String>,
    pub dept_id: Option<u64>,
    pub del_flag: Option<u8>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysRole {
    id,
    name,
    dept_id,
    del_flag,
    remark,
    creator,
    create_date,
    updater,
    update_date,
});

/**
*struct:SysRoleDataScope
*desc:角色部门关系表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_role_data_scope)]
#[derive(Clone, Debug)]
pub struct SysRoleDataScope {
    pub id: Option<i64>,
    pub role_id: Option<i64>,
    pub dept_id: Option<i64>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysRoleDataScope {
    id,
    role_id,
    dept_id,
    creator,
    create_date,
});

/**
*struct:
*desc:角色用户关系表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_role_user)]
#[derive(Clone, Debug)]
pub struct SysRoleUser {
    pub id: Option<i64>,
    pub role_id: Option<i64>,
    pub user_id: Option<i64>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysRoleUser {
    id,
    role_id,
    user_id,
    creator,
    create_date,
});

/**
*struct:SysRoleMenu
*desc:角色菜单关系表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_role_menu)]
#[derive(Clone, Debug)]
pub struct SysRoleMenu {
    pub id: Option<i64>,
    pub role_id: Option<i64>,
    pub menu_id: Option<i64>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysRoleMenu {
    id,
    role_id,
    menu_id,
    creator,
    create_date,
});

/**
*struct:SysMenu
*desc:菜单表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:sys_menu)]
#[derive(Clone, Debug)]
pub struct SysMenu {
    pub id: Option<i64>,
    pub pid: Option<i64>,
    pub url: Option<String>,
    pub name:Option<String>,
    pub menu_type: Option<u8>,
    pub icon: Option<String>,
    pub permissions: Option<String>,
    pub sort: Option<u64>,
    pub del_flag: Option<u8>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
    pub method:Option<String>
}
impl_field_name_method!(SysMenu {
    id,
    pid,
    url,
    menu_type,
    icon,
    permissions,
    sort,
    del_flag,
    creator,
    create_date,
    updater,
    update_date,
    method
});
/**
*struct:CommonField
*desc:所有表的公共字段 CRUD_SERVICE使用
*author:String
*email:348040933@qq.com
*/
#[derive(Clone, Debug)]
pub struct CommonField {
    pub id: Option<i64>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}
