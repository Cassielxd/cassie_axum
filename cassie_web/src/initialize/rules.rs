use cassie_config::config::ApplicationConfig;
use cassie_rules::core::rules::RulesContext;

use crate::APPLICATION_CONTEXT;

pub fn get_config() -> &'static ApplicationConfig {
    APPLICATION_CONTEXT.get::<ApplicationConfig>()
}

//初始话规则引擎
pub fn init_rules() {
    //创建默认规则对象
    let mut rule_context = RulesContext::default();
    rule_context.engine.register_fn("get_config", get_config);
    //加载所有规则
}
