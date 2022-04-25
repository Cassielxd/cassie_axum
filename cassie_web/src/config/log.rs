use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{Packer, RollingType};
use fast_log::plugin::packer::{GZipPacker, LZ4Packer, LogPacker, ZipPacker};
use std::time::Duration;

use crate::APPLICATION_CONTEXT;

use cassie_config::config::ApplicationConfig;

pub fn init_log() {
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //create log dir
    std::fs::create_dir_all(&cassie_config.log_dir());
    //initialize fast log
    fast_log::init(Config::new().console().file_split(
        &cassie_config.log_dir(),
        str_to_temp_size(&cassie_config.log_temp_size()),
        str_to_rolling(&cassie_config.log_rolling_type()),
        ZipPacker {},
    ))
    .unwrap();
}

fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> log::Level {
    return match arg {
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        "trace" => log::Level::Trace,
        "info" => log::Level::Info,
        "debug" => log::Level::Debug,
        _ => log::Level::Info,
    };
}
