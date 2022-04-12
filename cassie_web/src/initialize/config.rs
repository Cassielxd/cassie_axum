use crate::APPLICATION_CONTEXT;
use cassie_config::config::ApplicationConfig;
pub async fn init_config() {
    let yml_data = include_str!("../../application.yml");
    let mut config = ApplicationConfig::new(yml_data);
    let mut list = match config.admin_auth_list_api().clone() {
        Some(e) => e,
        None => Vec::new(),
    };
    //添加需要登录但是不需要权限的路由
    list.push("/user/info".to_string());
    list.push("/dict/type/all".to_string());
    list.push("/menu/nav".to_string());
    config.set_admin_auth_list_api(Some(list));
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}
