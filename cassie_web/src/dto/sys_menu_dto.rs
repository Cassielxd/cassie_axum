use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};

use crate::entity::sys_entitys::SysMenu;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysMenuDTO {
    pub id: Option<i64>,
    pub pid: Option<i64>,
    pub url: Option<String>,
    pub menu_type: Option<u8>,
    pub open_style: Option<u8>,
    pub icon: Option<String>,
    pub permissions: Option<String>,
    pub sort: Option<u64>,
    pub del_flag: Option<u8>,
    pub creator: Option<i64>,
    pub create_date: Option<DateTimeNative>,
    pub updater: Option<i64>,
    pub update_date: Option<DateTimeNative>,
}
impl_field_name_method!(SysMenuDTO {
    id,
    pid,
    url,
    menu_type,
    open_style,
    icon,
    permissions,
    sort,
    del_flag,
    creator,
    create_date,
    updater,
    update_date
});
impl Into<SysMenu> for SysMenuDTO {
    fn into(self) -> SysMenu {
        SysMenu {
            id: self.id,
            pid: self.pid,
            url: self.url,
            menu_type: self.menu_type,
            open_style: self.open_style,
            icon: self.icon,
            permissions: self.permissions,
            sort: self.sort,
            del_flag: self.del_flag,
            creator: self.creator,
            create_date: self.create_date,
            updater: self.updater,
            update_date: self.update_date,
        }
    }
}

impl From<SysMenu> for SysMenuDTO {
    fn from(arg: SysMenu) -> Self {
        Self {
            id: arg.id,
            pid: arg.pid,
            url: arg.url,
            menu_type: arg.menu_type,
            open_style: arg.open_style,
            icon: arg.icon,
            permissions: arg.permissions,
            sort: arg.sort,
            del_flag: arg.del_flag,
            creator: arg.creator,
            create_date: arg.create_date,
            updater: arg.updater,
            update_date: arg.update_date,
        }
    }
}
