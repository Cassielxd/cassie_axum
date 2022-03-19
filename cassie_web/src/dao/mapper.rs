

use crate::entity::sys_entitys::SysMenu;
use crate::RB;

//查询用户所有菜单
#[html_sql(RB, "cassie_web/mapper/menu_mapper.html")]
pub async fn user_menu_list(user_id: &str, t: &str) -> Option<Vec<SysMenu>> {
    todo!()
}
//查询所有菜单
#[html_sql(RB, "cassie_web/mapper/menu_mapper.html")]
pub async fn menu_list(t: &str) -> Option<Vec<SysMenu>> {
    todo!()
}
//根据id pid查询菜单
#[html_sql(RB, "cassie_web/mapper/menu_mapper.html")]
pub async fn get_menu_list_by_ids(ids: &Vec<i64>) -> Option<Vec<SysMenu>> {
    todo!() 
}
