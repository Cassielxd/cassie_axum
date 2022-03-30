use serde::{Deserialize, Serialize};

/**
*struct:SysUserQuery
*desc:用户列表查询参数
*author:String
*email:348040933
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysUserQuery {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub gender: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<String>,
    pub order_field: Option<String>,
}
/**
*struct:struct
*desc:字典项查询参数
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysDictQuery {
    pub id: Option<i64>,
    pub dict_type_id: Option<i64>,
    pub dict_type: Option<String>,
    pub dict_name: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<String>,
    pub order_field: Option<String>,
}
/**
*struct:SysParamsQuery
*desc:系统参数配置查询实体
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysParamsQuery {
    pub id: Option<i64>,
    pub param_code: Option<String>,
    pub param_type: Option<u32>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<String>,
    pub order_field: Option<String>,
}
/**
*struct:SysRoleQuery
*desc:角色查询
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleQuery {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub dept_id: Option<i64>,
    pub role_id: Option<i64>,
    pub user_id: Option<i64>,
    pub menu_id: Option<i64>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<String>,
    pub order_field: Option<String>,
}
/**
*struct:SysMenuQuery
*desc:菜单查询
*author:String
*email:348040933@qq.com
*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysMenuQuery {
    pub ids: Option<Vec<i64>>,
    pub pids: Option<Vec<i64>>,
    pub pid: Option<i64>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<String>,
    pub order_field: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AsiQuery {
    pub column_code: Option<String>,
    pub group_code: Option<String>,
    pub parent_group_code: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<String>,
    pub order_field: Option<String>,
}