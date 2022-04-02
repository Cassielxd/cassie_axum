use crate::service::crud_service::CrudService;
use crate::{ServiceContext, APPLICATION_CONTEXT};
use cached::proc_macro::cached;
use cassie_common::error::Result;
use cassie_domain::dto::sys_menu_dto::SysMenuDTO;
use cassie_domain::entity::sys_entitys::{CommonField, SysMenu};
use cassie_domain::request::tree::TreeService;
use cassie_domain::request::{RequestModel, SysMenuQuery};
use cassie_orm::dao::mapper::{menu_list, user_menu_list};
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
/**
*struct:SysMenuService
*desc:菜单基础服务
*author:String
*email:348040933@qq.com
*/
pub struct SysMenuService {}

impl SysMenuService {
    //保存或者更新
    pub async fn save_or_update(&self, dto: SysMenuDTO) {
        let mut entity: SysMenu = dto.into();
        //保存或更新菜单
        let id = if let Some(id) = entity.id {
            self.update_by_id(id.to_string(), &entity).await;
            id
        } else {
            entity.del_flag = Option::Some(0);
            let role_id = self.save(&mut entity).await;
            role_id.unwrap()
        };
    }
    //获取所有的菜单
    pub async fn menu_list(&self) -> Result<Vec<SysMenuDTO>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let result = menu_list(&mut rb.as_executor(), "").await.unwrap();
        Ok(self.build(result.unwrap()))
    }
}
impl Default for SysMenuService {
    fn default() -> Self {
        SysMenuService {}
    }
}
impl CrudService<SysMenu, SysMenuDTO, SysMenuQuery> for SysMenuService {
    fn get_wrapper(arg: &SysMenuQuery) -> Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut wrapper = rb.new_wrapper();
        if let Some(id_list) = &arg.ids {
            wrapper = wrapper.r#in(SysMenu::id(), id_list);
        }
        if let Some(id_list) = &arg.pids {
            wrapper = wrapper.r#in(SysMenu::pid(), id_list);
        }
        wrapper
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysMenu) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
impl TreeService<SysMenu, SysMenuDTO> for SysMenuService {
    fn set_children(&self, arg: &mut SysMenuDTO, childs: Option<Vec<SysMenuDTO>>) {
        arg.children = childs;
    }
}
//获取用户的菜单
#[cached(name = "USER_MENU_LIST", time = 60, result = true)]
pub async fn get_user_menu_list(uid: String, super_admin: i32) -> Result<Vec<SysMenuDTO>> {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();

    let result = if super_admin > 0 {
        menu_list(&mut rb.as_executor(), "0").await.unwrap()
    } else {
        user_menu_list(&mut rb.as_executor(), uid.as_str(), "0")
            .await
            .unwrap()
    };
    let context = APPLICATION_CONTEXT.get::<ServiceContext>();
    Ok(context.sys_menu_service.build(result.unwrap()))
}
