use crate::config::config::ApplicationConfig;

pub mod config;
pub mod log;

lazy_static! {
    pub static ref CASSIE_CONFIG: ApplicationConfig = ApplicationConfig::default();
}
