use crate::entity::sys_entitys::SysMenu;
use crate::RB;
#[py_sql(RB,"select t3.* from sys_role_user t1
        left join sys_role_menu t2 on t1.role_id = t2.role_id
        left join sys_menu t3 on t2.menu_id = t3.id
        where t1.user_id = #{user_id} and t3.del_flag = 0
        if t !='':
           and t3.menu_type = #{t}
       order by t3.sort asc
        ")]
pub async fn user_menu_List(language: &str, user_id: &str, t: &str) -> Option<Vec<SysMenu>> {
    todo!()
}
#[py_sql(RB,"select t1.*
            from sys_menu t1 where t1.del_flag = 0
        if t !='':
           and t1.menu_type = #{t}
       order by t1.sort asc
        ")]
pub async fn menu_List(language: &str,t: &str) -> Option<Vec<SysMenu>> {
    todo!()
}

