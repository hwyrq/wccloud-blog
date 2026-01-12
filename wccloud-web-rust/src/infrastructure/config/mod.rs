//! author wcz
use config::Config;
use serde::Deserialize;
use std::sync::RwLock;

pub mod ibatis_config;
pub mod nacos_config;
pub mod redis_config;
pub mod sea_config;
pub mod snow_flake_config;

static CONF: RwLock<Option<Config>> = RwLock::new(None);

///初始化文件配置
pub fn init_config_file() {
    let app_yml = Config::builder()
        .add_source(config::File::with_name("application.yml"))
        .build()
        .unwrap();
    let mut conf = CONF.write().unwrap();
    *conf = Some(app_yml);
}

///设置配置（用于 nacos 动态更新）
pub fn set_config(config: Config) {
    let mut conf = CONF.write().unwrap();
    *conf = Some(config);
}

///获取配置
pub fn get_config_value<'de, T: Deserialize<'de>>(key: &str) -> T {
    let conf = CONF.read().unwrap();
    conf.as_ref()
        .expect("Config not initialized")
        .get(key)
        .expect(&format!("Config key '{}' not found", key))
}
