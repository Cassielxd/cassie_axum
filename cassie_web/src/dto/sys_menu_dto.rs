use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

use crate::{entity::sys_entitys::SysMenu, utils::tree::TreeModel};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysMenuDTO {
    pub id: Option<i64>,
    pub pid: Option<i64>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub menu_type: Option<u8>,
    pub icon: Option<String>,
    pub permissions: Option<String>,
    pub sort: Option<u64>,
    pub del_flag: Option<u8>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub children: Option<Vec<SysMenuDTO>>,
}
impl_field_name_method!(SysMenuDTO {
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
    update_date
});
impl TreeModel for SysMenuDTO {
    fn get_pid(&self) -> Option<String> {
        Some(self.pid.clone().unwrap().to_string())
    }

    fn get_id(&self) -> Option<String> {
        Some(self.id.clone().unwrap().to_string())
    }
}

impl Into<SysMenu> for SysMenuDTO {
    fn into(self) -> SysMenu {
        SysMenu {
            id: self.id,
            pid: self.pid,
            url: self.url,
            name: self.name,
            menu_type: self.menu_type,
            icon: self.icon,
            permissions: self.permissions,
            sort: self.sort,
            del_flag: self.del_flag,
            creator: self.creator,
            create_date: self.create_date,
            updater: self.updater,
            update_date: self.update_date,
            method: self.method,
            path: self.path,
        }
    }
}

impl From<SysMenu> for SysMenuDTO {
    fn from(arg: SysMenu) -> Self {
        Self {
            id: arg.id,
            pid: arg.pid,
            url: arg.url,
            name: arg.name,
            menu_type: arg.menu_type,
            icon: arg.icon,
            permissions: arg.permissions,
            sort: arg.sort,
            del_flag: arg.del_flag,
            creator: arg.creator,
            create_date: arg.create_date,
            updater: arg.updater,
            update_date: arg.update_date,
            method: arg.method,
            path: arg.path,
            children: Some(Vec::<SysMenuDTO>::new()),
        }
    }
}
