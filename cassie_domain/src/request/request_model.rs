use serde::{Deserialize, Serialize};

/**
*struct:SysUserQuery
*desc:用户列表查询参数
*author:String
*email:348040933
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysUserQuery {
    id: Option<i64>,
    username: Option<String>,
    gender: Option<String>,
    page: Option<u64>,
    limit: Option<u64>,
    order: Option<String>,
    order_field: Option<String>,
}
/**
*struct:struct
*desc:字典项查询参数
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysDictQuery {
    id: Option<i64>,
    dict_type_id: Option<i64>,
    dict_type: Option<String>,
    dict_name: Option<String>,
    page: Option<u64>,
    limit: Option<u64>,
    order: Option<String>,
    order_field: Option<String>,
}
/**
*struct:SysParamsQuery
*desc:系统参数配置查询实体
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysParamsQuery {
    id: Option<i64>,
    param_code: Option<String>,
    param_type: Option<u32>,
    page: Option<u64>,
    limit: Option<u64>,
    order: Option<String>,
    order_field: Option<String>,
}
/**
*struct:SysRoleQuery
*desc:角色查询
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysRoleQuery {
    id: Option<u64>,
    name: Option<String>,
    dept_id: Option<i64>,
    role_id: Option<i64>,
    user_id: Option<i64>,
    menu_id: Option<i64>,
    page: Option<u64>,
    limit: Option<u64>,
    order: Option<String>,
    order_field: Option<String>,
}
/**
*struct:SysMenuQuery
*desc:菜单查询
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysMenuQuery {
    ids: Option<Vec<i64>>,
    pids: Option<Vec<i64>>,
    pid: Option<i64>,
    page: Option<u64>,
    limit: Option<u64>,
    order: Option<String>,
    order_field: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct AsiQuery {
    column_code: Option<String>,
    group_code: Option<String>,
    parent_group_code: Option<String>,
    page: Option<u64>,
    limit: Option<u64>,
    order: Option<String>,
    order_field: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct LogQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct EventQuery {}
