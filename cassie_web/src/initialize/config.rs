use crate::APPLICATION_CONTEXT;
use cassie_config::config::ApplicationConfig;

pub async fn init_config() {
    let yml_data = include_str!("../../application.yml");
    let config = ApplicationConfig::new(yml_data);

    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}
