use crate::entity::sys_entitys::{SysRole, SysRoleDataScope, SysRoleMenu, SysRoleUser};
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

/**
 * 角色dto
 */
#[derive(Clone, Debug, Serialize, Deserialize, Validate, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysRoleDTO {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub remark: Option<String>,
    pub dept_id: Option<u64>,
    pub del_flag: Option<u8>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
    pub menuid_list: Option<Vec<i64>>,
}

impl Into<SysRole> for SysRoleDTO {
    fn into(self) -> SysRole {
        SysRole {
            id: self.id,
            name: self.name,
            remark: self.remark,
            dept_id: self.dept_id,
            del_flag: self.del_flag,
            creator: self.creator,
            create_date: self.create_date,
            updater: self.updater,
            update_date: self.update_date,
        }
    }
}

impl From<SysRole> for SysRoleDTO {
    fn from(arg: SysRole) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            remark: arg.remark,
            dept_id: arg.dept_id,
            del_flag: arg.del_flag,
            creator: arg.creator,
            create_date: arg.create_date,
            updater: arg.updater,
            update_date: arg.update_date,
            menuid_list: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysRoleDataScopeDTO {
    pub id: Option<i64>,
    pub role_id: Option<i64>,
    pub dept_id: Option<i64>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}

impl Into<SysRoleDataScope> for SysRoleDataScopeDTO {
    fn into(self) -> SysRoleDataScope {
        SysRoleDataScope {
            id: self.id,
            role_id: self.role_id,
            dept_id: self.dept_id,
            creator: self.creator,
            create_date: self.create_date,
        }
    }
}

impl From<SysRoleDataScope> for SysRoleDataScopeDTO {
    fn from(arg: SysRoleDataScope) -> Self {
        Self {
            id: arg.id,
            role_id: arg.role_id,
            dept_id: arg.dept_id,
            creator: arg.creator,
            create_date: arg.create_date,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysRoleMenuDTO {
    pub id: Option<i64>,
    pub role_id: Option<i64>,
    pub menu_id: Option<i64>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}

impl Into<SysRoleMenu> for SysRoleMenuDTO {
    fn into(self) -> SysRoleMenu {
        SysRoleMenu {
            id: self.id,
            role_id: self.role_id,
            menu_id: self.menu_id,
            creator: self.creator,
            create_date: self.create_date,
        }
    }
}

impl From<SysRoleMenu> for SysRoleMenuDTO {
    fn from(arg: SysRoleMenu) -> Self {
        Self {
            id: arg.id,
            role_id: arg.role_id,
            menu_id: arg.menu_id,
            creator: arg.creator,
            create_date: arg.create_date,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysRoleUserDTO {
    pub id: Option<i64>,
    pub role_id: Option<i64>,
    pub user_id: Option<i64>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
}

impl Into<SysRoleUser> for SysRoleUserDTO {
    fn into(self) -> SysRoleUser {
        SysRoleUser {
            id: self.id,
            role_id: self.role_id,
            user_id: self.user_id,
            creator: self.creator,
            create_date: self.create_date,
        }
    }
}

impl From<SysRoleUser> for SysRoleUserDTO {
    fn from(arg: SysRoleUser) -> Self {
        Self {
            id: arg.id,
            role_id: arg.role_id,
            user_id: arg.user_id,
            creator: arg.creator,
            create_date: arg.create_date,
        }
    }
}
