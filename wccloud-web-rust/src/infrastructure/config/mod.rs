//! author wcz
use config::Config;
use serde::Deserialize;

pub mod ibatis_config;
pub mod nacos_config;

pub mod sea_config;
pub mod redis_config;
pub mod snow_flake_config;

pub static mut CONF: Option<Config> = None;

///初始化文件配置
pub fn init_config_file() {
    let  app_yml = Config::builder()
        .add_source(config::File::with_name("application.yml"))
        .build().unwrap();
    unsafe { CONF = Some(app_yml); }
}
///获取配置
pub fn get_config_value<'de, T: Deserialize<'de>>(key: &str)-> T {
    unsafe { CONF.as_mut().unwrap().get(key).unwrap() }
}