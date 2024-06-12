//! author wcz
use actix_web::{App, HttpServer};

use crate::controller::anonymous_blog_controller::anonymous_blog_controller;
use crate::controller::web_blog_controller::web_blog_controller;
use crate::controller::web_label_controller::web_label_controller;
use crate::controller::web_type_controller::web_type_controller;
use crate::infrastructure::config::{get_config_value, init_config_file};
use crate::infrastructure::config::nacos_config::init_nacos;
use crate::infrastructure::config::snow_flake_config::init_snow_flake;

mod infrastructure;

mod domain;

mod application;

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //初始化日志
    tracing_subscriber::fmt::init();
    //初始化配置文件
    init_config_file();
    //初始化雪花ID
    init_snow_flake().await;
    //初始化nacos discovery ,nacos config,获取到在线配置以及配置变更 后初始化 sea_orm,redis_pool
    init_nacos().await;
    
    HttpServer::new(|| {
        App::new()
            .configure(web_blog_controller)
            .configure(web_label_controller)
            .configure(web_type_controller)
            .configure(anonymous_blog_controller)
    }).bind(("0.0.0.0", get_config_value("server.port")))?.run().await
}


#[test]
fn test1() {}
