use std::collections::HashMap;

use crate::dao::mapper::{menu_List, user_menu_List};
use crate::dto::sys_menu_dto::SysMenuDTO;
use crate::entity::sys_entitys::{CommonField, SysMenu};
use crate::request::SysMenuQuery;
use crate::service::crud_service::CrudService;
use crate::{CONTEXT, RB, REQUEST_CONTEXT};
use cassie_common::error::Result;
use rbatis::wrapper::Wrapper;

/**
*struct:SysMenuService
*desc:菜单基础服务
*author:String
*email:348040933@qq.com
*/
pub struct SysMenuService {}

impl SysMenuService {
    pub async fn save_or_update(&self, dto: SysMenuDTO) {
        let mut entity: SysMenu = dto.into();
        //保存或更新菜单
        let id = if let Some(id) = entity.id {
            self.update_by_id(id.to_string(), &entity).await;
            id
        } else {
            let role_id = self.save(&mut entity).await;
            role_id.unwrap()
        };
    }

    pub async fn menu_list(&self) -> Result<Vec<SysMenuDTO>> {
    
        let result = menu_List("zh-CN", "").await.unwrap();
        Ok(self.build(result.unwrap()))
    }

    pub async fn get_user_menu_list(&self) -> Result<Vec<SysMenuDTO>> {
        let tls = REQUEST_CONTEXT.clone();
        let (uid, super_admin) = if let Some(a) = tls.get() {
            (a.uid, a.super_admin)
        } else {
            (0, 0)
        };
        let result = if super_admin > 0 {
            menu_List("zh-CN", "0").await.unwrap()
        } else {
            user_menu_List("zh-CN", uid.to_string().as_str(), "0")
                .await
                .unwrap()
        };
        Ok(self.build(result.unwrap()))
    }

    fn build(&self,menus:Vec<SysMenu>)->Vec<SysMenuDTO>{
        let mut result = HashMap::with_capacity(menus.capacity());
        let mut data = vec![];
        for x in menus {
            result.insert(x.id.clone().unwrap_or_default(), x);
        }
        for (k, v) in &result {
            if v.pid.unwrap()==0 {
                let mut top = SysMenuDTO::from(v.clone());
                self.loop_find_childs(&mut top,&result);
                data.push(top);
            }
        }
        data
    }
    
    fn loop_find_childs(&self,arg: &mut SysMenuDTO, all: &HashMap<i64, SysMenu>){
        let mut childs = vec![];
        for (key, x) in all {
            if x.pid.is_some() && x.pid.eq(&arg.id) {
                let mut item = SysMenuDTO::from(x.clone());
                self.loop_find_childs(&mut item, all);
                childs.push(item);
            }
        }
        if !childs.is_empty() {
            arg.children = Some(childs);
        }
    }
}
impl Default for SysMenuService {
    fn default() -> Self {
        SysMenuService {}
    }
}
impl CrudService<SysMenu, SysMenuDTO, SysMenuQuery> for SysMenuService {
    fn get_wrapper(arg: &SysMenuQuery) -> Wrapper {
        let mut wrapper = RB.new_wrapper();
        if let Some(id_list) = &arg.ids {
            wrapper = wrapper.r#in(SysMenu::id(), id_list);
        }
        wrapper
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysMenu) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}

