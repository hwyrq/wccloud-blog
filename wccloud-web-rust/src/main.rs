//! author wcz
use std::thread;
use std::time::Duration;
use actix_web::{App, HttpServer, };
use tokio::join;

use crate::controller::anonymous_blog_controller::anonymous_blog_controller;
use crate::controller::web_blog_controller::web_blog_controller;
use crate::controller::web_label_controller::web_label_controller;
use crate::controller::web_type_controller::web_type_controller;
use crate::infrastructure::config::nacos_config::init_nacos;
use crate::infrastructure::config::redis_config::init_redis;
use crate::infrastructure::config::sea_config::init_sea;
use crate::infrastructure::config::snow_flake_config::init_snow_flake;

mod infrastructure;

mod domain;

mod application;

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    tracing_subscriber::fmt::init();
    join!(init_nacos(),init_sea(),init_redis(),init_snow_flake());

    HttpServer::new(|| {
        App::new()
            .configure(web_blog_controller)
            .configure(web_label_controller)
            .configure(web_type_controller)
            .configure(anonymous_blog_controller)
    }).bind(("127.0.0.1", 8085))?.run().await
}



#[test]
fn test1(){
  
}
