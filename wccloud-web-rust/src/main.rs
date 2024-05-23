//! author wcz
use actix_web::{App, HttpServer, };
use chrono::NaiveDateTime;
use tokio::join;

use crate::controller::anonymous_blog_controller::anonymous_blog_controller;
use crate::controller::web_blog_controller::web_blog_controller;
use crate::controller::web_label_controller::web_label_controller;
use crate::controller::web_type_controller::web_type_controller;
use crate::infrastructure::config::nacos_config::init_nacos;
use crate::infrastructure::config::redis_config::init_redis;
use crate::infrastructure::config::sea_config::init_sea;

mod infrastructure;

mod domain;

mod application;

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    tracing_subscriber::fmt::init();
    join!(init_nacos(),init_sea(),init_redis());

    HttpServer::new(|| {
        App::new()
            .configure(web_blog_controller)
            .configure(web_label_controller)
            .configure(web_type_controller)
            .configure(anonymous_blog_controller)
    }).bind(("127.0.0.1", 8085))?.run().await
}

#[test]
/// 测试一下雪花id
fn test1() {
    let time = NaiveDateTime::parse_from_str("1970-01-01 00:00:11", "%Y-%m-%d %H:%M:%S");
    println!("{:?}", time.unwrap().and_utc().timestamp());
    let mut settings = spaceflake::GeneratorSettings::default();
    settings.base_epoch = 0;
    println!("{:?}", settings);
    println!("{:?}", spaceflake::GeneratorSettings::default());
    println!("{:?}", spaceflake::generate(settings).unwrap().id);
    println!("{:?}", spaceflake::generate(spaceflake::GeneratorSettings::default()).unwrap().id);
}
