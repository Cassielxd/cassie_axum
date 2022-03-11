use super::crud_service::CrudService;
use crate::cici_casbin::CASBIN_CONTEXT;
use crate::dao::mapper::get_menu_List_by_ids;
use crate::entity::sys_entitys::CommonField;
use crate::request::SysMenuQuery;
use crate::{
    dto::sys_role_dto::SysRoleMenuDTO, entity::sys_entitys::SysRoleMenu, request::SysRoleQuery,
};
use crate::{CONTEXT, RB, REQUEST_CONTEXT};
use casbin::MgmtApi;
use rbatis::{crud::CRUD, DateTimeNative};

/**
*struct:SysRoleMenuService
*desc:角色和菜单的关系
*author:String
*email:348040933@qq.com
*/
pub struct SysRoleMenuService {}
impl Default for SysRoleMenuService {
    fn default() -> Self {
        SysRoleMenuService {}
    }
}
impl SysRoleMenuService {
    pub async fn get_menu_id_list(&self, role_id: i64) -> Vec<i64> {
        //构建查询条件
        let wrapper = RB.new_wrapper().eq(SysRoleMenu::role_id(), role_id);
        //执行查询
        let list = RB.fetch_list_by_wrapper::<SysRoleMenu>(wrapper).await;

        //将Entity实体转换成 Vo对象 返回
        if let Ok(ls) = list {
            return ls
                .iter()
                .map(|f| f.menu_id.clone().unwrap_or_default())
                .collect::<Vec<i64>>();
        } else {
            return Vec::<i64>::new();
        }
    }
    pub async fn save_or_update(&self, role_id: i64, menu_id_list: Option<Vec<i64>>) {
        //先删除角色菜单关系
        self.delete_by_role_id(role_id).await;
        /*添加到casbin权限表里*/
        let query = SysMenuQuery {
            ids: None,
            pids: menu_id_list.clone(),
            pid: None,
            page: None,
            limit: None,
            order: None,
            order_field: None,
        };
        let tls = REQUEST_CONTEXT.clone();
        let (creator, agency_code) = if let Some(a) = tls.get() {
            (a.uid as i64, a.agency_code.clone())
        } else {
            (0, "".to_string())
        };
       let r_list = get_menu_List_by_ids(&menu_id_list.clone().unwrap()).await.unwrap();
        let menus = CONTEXT.sys_menu_service.list(&query).await;
            let mut rules = vec![];
            let mut vec = Vec::new();
            for menu in r_list.unwrap() {
                if let Some(mlist) = menu_id_list.clone() {
                     if mlist.contains(&menu.id.clone().unwrap()){
                        vec.push(SysRoleMenu {
                            id: None,
                            role_id: Some(role_id),
                            menu_id: menu.id.clone(),
                            creator: Some(creator),
                            create_date: Some(DateTimeNative::now()),
                        });
                     }
                }
                if menu.method.is_some()&&!menu.method.clone().unwrap().is_empty()  {
                    rules.push(vec![
                        role_id.to_string(),                            //角色编码
                        agency_code.clone(),                            //租户
                        menu.path.unwrap_or_default(),                  //资源
                        menu.method.unwrap_or_default().to_uppercase(), //请求方式
                    ]);
                }
            }
            //保存角色菜单关系
            self.save_batch(&vec).await;
            //同步到权限框架casbin
            let cached_enforcer = CASBIN_CONTEXT.enforcer.clone();
            let mut lock = cached_enforcer.write().await;
            lock.add_policies(rules).await;
            drop(lock);
    }

    pub async fn delete_by_role_id(&self, role_id: i64) {
        let tls = REQUEST_CONTEXT.clone();
        let (creator, agency_code) = if let Some(a) = tls.get() {
            (a.uid as i64, a.agency_code.clone())
        } else {
            (0, "".to_string())
        };
        self.del_by_column(SysRoleMenu::role_id(), &role_id.to_string())
            .await;
        let cached_enforcer = CASBIN_CONTEXT.enforcer.clone();
        let mut lock = cached_enforcer.write().await;
        lock.remove_named_policy("p",vec![role_id.to_string(),agency_code.clone()])
            .await;
        drop(lock);
    }

    pub async fn delete_by_menu_id(&self, menu_id: i64) {
        self.del_by_column(SysRoleMenu::menu_id(), &menu_id.to_string())
            .await;
    }
}

impl CrudService<SysRoleMenu, SysRoleMenuDTO, SysRoleQuery> for SysRoleMenuService {
    fn get_wrapper(arg: &SysRoleQuery) -> rbatis::wrapper::Wrapper {
        RB.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysRoleMenu) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
