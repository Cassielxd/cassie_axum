use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

use crate::entity::sys_entitys::SysMenu;
use crate::request::tree::TreeModel;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysMenuDTO {
    id: Option<i64>,
    pid: Option<i64>,
    url: Option<String>,
    name: Option<String>,
    menu_type: Option<u8>,
    icon: Option<String>,
    permissions: Option<String>,
    sort: Option<u64>,
    del_flag: Option<u8>,
    creator: Option<i64>,
    create_date: Option<DateTimeNative>,
    updater: Option<i64>,
    update_date: Option<DateTimeNative>,
    method: Option<String>,
    path: Option<String>,
    children: Option<Vec<SysMenuDTO>>,
}

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
