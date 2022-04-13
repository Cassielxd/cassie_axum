use super::crud_service::CrudService;
use super::sys_menu_service::SysMenuService;
use crate::cici_casbin::casbin_service::CasbinService;
use crate::middleware::auth::get_local;
use crate::APPLICATION_CONTEXT;
use casbin::MgmtApi;
use cassie_domain::entity::sys_entitys::CommonField;
use cassie_domain::request::SysMenuQuery;
use cassie_domain::{
    dto::sys_role_dto::SysRoleMenuDTO, entity::sys_entitys::SysRoleMenu, request::SysRoleQuery,
};
use cassie_orm::dao::mapper::get_menu_list_by_ids;
use rbatis::rbatis::Rbatis;
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
    //根据角色id获取菜单
    pub async fn get_menu_id_list(&self, role_id: i64) -> Vec<i64> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        //构建查询条件
        let wrapper = rb.new_wrapper().eq(SysRoleMenu::role_id(), role_id);
        //执行查询
        let list = rb.fetch_list_by_wrapper::<SysRoleMenu>(wrapper).await;

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
    //保存或者更新角色与菜单的关系
    pub async fn save_or_update(&self, role_id: i64, menu_id_list: Option<Vec<i64>>) {
        //先删除角色菜单关系
        self.delete_by_role_id(role_id).await;
        /*添加到casbin权限表里*/
        let mut query = SysMenuQuery::default();
        query.set_pids(menu_id_list.clone());
        let request_model = get_local().unwrap();
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let sys_menu_service = APPLICATION_CONTEXT.get::<SysMenuService>();
        let r_list = get_menu_list_by_ids(&mut rb.as_executor(), &menu_id_list.clone().unwrap())
            .await
            .unwrap();
        let menus = sys_menu_service.list(&query).await;
        let mut rules = vec![];
        let mut vec = Vec::new();
        for menu in r_list.unwrap() {
            if let Some(mlist) = menu_id_list.clone() {
                if mlist.contains(&menu.id.clone().unwrap()) {
                    vec.push(SysRoleMenu {
                        id: None,
                        role_id: Some(role_id),
                        menu_id: menu.id.clone(),
                        creator: Some(request_model.uid().clone()),
                        create_date: Some(DateTimeNative::now()),
                    });
                }
            }
            if menu.method.is_some() && !menu.method.clone().unwrap().is_empty() {
                rules.push(vec![
                    role_id.to_string(),                            //角色编码
                    request_model.agency_code().clone(),            //租户
                    menu.path.unwrap_or_default(),                  //资源
                    menu.method.unwrap_or_default().to_uppercase(), //请求方式
                ]);
            }
        }
        //保存角色菜单关系
        self.save_batch(&vec).await;
        //同步到权限框架casbin
        let cached_enforcer = APPLICATION_CONTEXT.get::<CasbinService>().enforcer.clone();
        let mut lock = cached_enforcer.write().await;
        lock.add_policies(rules).await;
        drop(lock);
    }
    //删除角色与菜单的关系
    pub async fn delete_by_role_id(&self, role_id: i64) {
        let request_model = get_local().unwrap();
        self.del_by_column(SysRoleMenu::role_id(), &role_id.to_string())
            .await;
        let cached_enforcer = APPLICATION_CONTEXT.get::<CasbinService>().enforcer.clone();
        let mut lock = cached_enforcer.write().await;
        lock.remove_named_policy(
            "p",
            vec![role_id.to_string(), request_model.agency_code().clone()],
        )
        .await;
        drop(lock);
    }
    //批量删除
    pub async fn delete_by_menu_id(&self, menu_id: i64) {
        self.del_by_column(SysRoleMenu::menu_id(), &menu_id.to_string())
            .await;
    }
}

impl CrudService<SysRoleMenu, SysRoleMenuDTO, SysRoleQuery> for SysRoleMenuService {
    fn get_wrapper(arg: &SysRoleQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(&self, common: CommonField, data: &mut SysRoleMenu) {
        data.id = common.id;
        data.creator = common.creator;
        data.create_date = common.create_date;
    }
}
