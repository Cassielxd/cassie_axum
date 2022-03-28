use crate::dao::mapper::{menu_list, user_menu_list};
use crate::dto::sys_menu_dto::SysMenuDTO;
use crate::entity::sys_entitys::{CommonField, SysMenu};
use crate::request::{RequestModel, SysMenuQuery};
use crate::service::crud_service::CrudService;
use crate::utils::tree::TreeService;
use crate::CONTAINER;
use cassie_common::error::Result;
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

    pub async fn menu_list(&self) -> Result<Vec<SysMenuDTO>> {
        let  RB = CONTAINER.get::<Rbatis>();
        let result = menu_list(&mut RB.as_executor(),"").await.unwrap();
        Ok(self.build(result.unwrap()))
    }

    pub async fn get_user_menu_list(&self) -> Result<Vec<SysMenuDTO>> {
        let  RB = CONTAINER.get::<Rbatis>();
        let request_model = CONTAINER.get_local::<RequestModel>();
        let result = if request_model.super_admin > 0 {
            menu_list(&mut RB.as_executor(), "0").await.unwrap()
        } else {
            user_menu_list(&mut RB.as_executor(), request_model.uid.to_string().as_str(), "0")
                .await
                .unwrap()
        };

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
        let RB = CONTAINER.get::<Rbatis>();
        let mut wrapper = RB.new_wrapper();
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
