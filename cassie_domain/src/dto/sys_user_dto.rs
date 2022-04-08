use crate::entity::sys_entitys::SysUser;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;
#[derive(Clone, Debug, Serialize, Validate, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SysUserDTO {
    id: Option<i64>,
    #[validate(required)]
    username: Option<String>,
    #[validate(length(min = 6, message = "密码最少6个字符"))]
    password: Option<String>,
    real_name: Option<String>,
    head_url: Option<String>,
    gender: Option<u8>,
    #[validate(email)]
    email: Option<String>,
    #[validate(required)]
    mobile: Option<String>,
    #[validate(required)]
    role_id: Option<i64>,
    dept_id: Option<i32>,
    super_admin: Option<i32>,
    agency_code: Option<String>,
    remark: Option<String>,
    status: Option<i32>,
    del_flag: Option<i32>,
    creator: Option<i64>,
    create_date: Option<rbatis::DateTimeNative>,
    updater: Option<i64>,
    update_date: Option<rbatis::DateTimeNative>,
}

impl Into<SysUser> for SysUserDTO {
    fn into(self) -> SysUser {
        SysUser {
            id: self.id,
            username: self.username,
            password: self.password,
            real_name: self.real_name,
            head_url: self.head_url,
            gender: self.gender,
            email: self.email,
            mobile: self.mobile,
            dept_id: self.dept_id,
            super_admin: self.super_admin,
            remark: self.remark,
            status: self.status,
            del_flag: self.del_flag,
            creator: self.creator,
            create_date: self.create_date,
            updater: self.updater,
            update_date: self.update_date,
            agency_code: self.agency_code,
        }
    }
}

impl From<SysUser> for SysUserDTO {
    fn from(arg: SysUser) -> Self {
        Self {
            id: arg.id,
            username: arg.username,
            password: arg.password,
            real_name: arg.real_name,
            head_url: arg.head_url,
            gender: arg.gender,
            email: arg.email,
            mobile: arg.mobile,
            role_id: None,
            dept_id: arg.dept_id,
            super_admin: arg.super_admin,
            remark: arg.remark,
            status: arg.status,
            del_flag: arg.del_flag,
            creator: arg.creator,
            create_date: arg.create_date,
            updater: arg.updater,
            update_date: arg.update_date,
            agency_code: arg.agency_code,
        }
    }
}
