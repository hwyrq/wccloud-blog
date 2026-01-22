//! Gateway application for wccloud-blog

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use crate::application::http_handler::handle_proxy_request;
use crate::application::route_table::RouteTable;

mod application;
mod domain;
mod infrastructure;

// 全局状态结构
pub struct AppState {
    pub route_table: RouteTable,
    pub http_client: reqwest::Client,
    pub naming_service: Arc<nacos_sdk::api::naming::NamingService>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logging();

    let app_data = initialize_app_data().await;

    let server_port = crate::infrastructure::config::get_server_port();

    // 初始化配置文件 (通常在Nacos初始化后)
    crate::infrastructure::config::init_config_file();

    // 启动HTTP服务器
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .default_service(web::route().to(handle_proxy_request)) // 处理所有未匹配的路由
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}

// 设置日志
fn setup_logging() {
    use time::{format_description, UtcOffset};
    use tracing_subscriber::fmt::time::OffsetTime;
    let format = "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]";
    tracing_subscriber::fmt()
        .with_timer(OffsetTime::new(
            UtcOffset::current_local_offset().unwrap(),
            format_description::parse(format).unwrap(),
        ))
        .init();
}

// 初始化应用数据
async fn initialize_app_data() -> web::Data<AppState> {
    // 首先初始化路由表用于Nacos初始化
    let temp_route_table = RouteTable::new();
    // 初始化Nacos配置和发现服务
    crate::infrastructure::config::nacos_config::init_nacos(&temp_route_table).await;

    // 初始化路由表（从配置加载初始路由）
    let route_table = RouteTable::new();
    {
        let config = crate::infrastructure::config::get_config();
        for route in config.routes.iter().cloned() {
            route_table.add_route(route);
        }
    }

    // 获取命名服务实例（通过共享实例）
    let naming_service = get_shared_naming_service()
        .expect("Failed to get naming service, make sure Nacos initialization completed");

    // 初始化全局状态
    web::Data::new(AppState {
        route_table,
        http_client: reqwest::Client::new(),
        naming_service: Arc::new(naming_service),
    })
}

// 获取共享命名服务实例的辅助函数
fn get_shared_naming_service() -> Option<nacos_sdk::api::naming::NamingService> {
    crate::infrastructure::config::get_naming_service()
}
