use crate::dto::sys_menu_dto::SysMenuDTO;
use crate::entity::sys_entitys::{CommonField, SysMenu};
use crate::request::SysMenuQuery;
use crate::service::crud_service::CrudService;
use crate::CONTEXT;

use rbatis::wrapper::Wrapper;
/**
*struct:SysMenuService
*desc:菜单基础服务
*author:String
*email:348040933@qq.com
*/
pub struct SysMenuService {}

impl SysMenuService {
   pub async fn save_or_update(&self,dto:SysMenuDTO){
    let mut entity:SysMenu = dto.into();
         //保存或更新菜单
        let id = if let Some(id) = entity.id {
            self.update_by_id(id.to_string(), &entity).await;
            id
        } else {
            let role_id = self.save(&mut entity).await;
            role_id.unwrap()
        };
   }
}
impl  Default for SysMenuService {
    fn default() -> Self {
        SysMenuService{}
    }
}
impl CrudService<SysMenu, SysMenuDTO, SysMenuQuery> for SysMenuService {
    fn get_wrapper(arg: &SysMenuQuery) -> Wrapper {
        let mut wrapper = CONTEXT.rbatis.new_wrapper();
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
