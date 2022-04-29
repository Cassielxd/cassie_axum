use cassie_domain::entity::sys_entitys::SysMenu;
use rbatis::executor::RbatisExecutor;

//查询用户所有菜单
#[html_sql("cassie_orm/src/mapper/menu_mapper.html")]
pub async fn user_menu_list(rb: &mut RbatisExecutor<'_, '_>, user_id: &str, t: &str, agency_code: &str) -> Option<Vec<SysMenu>> {
    todo!()
}
//查询所有菜单
#[html_sql("cassie_orm/src/mapper/menu_mapper.html")]
pub async fn menu_list(rb: &mut RbatisExecutor<'_, '_>, t: &str) -> Option<Vec<SysMenu>> {
    todo!()
}
//根据id pid查询菜单
#[html_sql("cassie_orm/src/mapper/menu_mapper.html")]
pub async fn get_menu_list_by_ids(rb: &mut RbatisExecutor<'_, '_>, ids: &Vec<i64>) -> Option<Vec<SysMenu>> {
    todo!()
}
