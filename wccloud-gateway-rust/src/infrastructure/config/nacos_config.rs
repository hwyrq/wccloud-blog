use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, ConfigServiceBuilder};

use nacos_sdk::api::constants;
use nacos_sdk::api::naming::ServiceInstance;
use nacos_sdk::api::props::ClientProps;
use tracing::{error, info};

use std::sync::Arc;

use crate::application::route_table::RouteTable;
use crate::domain::models::{FilterConfig, PredicateConfig, RouteConfig};

use crate::infrastructure::config::get_config;

use serde_json;
use serde_yaml;

// 定义配置变更监听器
struct MyConfigChangeListener {
    route_table: RouteTable,
}

impl ConfigChangeListener for MyConfigChangeListener {
    fn notify(&self, config_resp: ConfigResponse) {
        info!("Refreshing config from Nacos");
        // 解析新配置并更新路由
        if let Some(routes) = parse_routes_from_config(config_resp.content()) {
            self.route_table.update_all_routes(routes);
            info!("Routes updated from new config");
        }
    }
}

// 初始化Nacos配置和服务发现

pub async fn init_nacos(route_table: &RouteTable) {
    let config = get_config();

    let server_addr = config.spring_config.cloud.nacos_config.server_addr.clone();
    let namespace = config
        .spring_config
        .cloud
        .nacos_config
        .discovery
        .namespace
        .clone();
    let service_name = config.spring_config.application.name.clone();

    let config_name = config.spring_config.cloud.nacos_config.config.name.clone();
    let file_extension = config
        .spring_config
        .cloud
        .nacos_config
        .config
        .file_extension
        .clone();
    // 构造Data ID (例如: "wccloud-shared.yaml")
    let data_id = config_name + "." + &file_extension;

    info!(
        "Initializing Nacos configuration. server_addr={}, namespace={}, data_id={}",
        server_addr, namespace, data_id
    );

    // 创建客户端属性

    let client_props = ClientProps::new()
        .server_addr(server_addr)
        .namespace(namespace)
        .auth_username(config.spring_config.cloud.nacos_config.username.clone())
        .auth_password(config.spring_config.cloud.nacos_config.password.clone());

    // 创建配置变更监听器
    let config_listener = Arc::new(MyConfigChangeListener {
        route_table: route_table.clone(),
    });

    // 构建配置服务

    let config_service = match ConfigServiceBuilder::new(client_props.clone()).build() {
        Ok(service) => service,
        Err(e) => {
            error!("Failed to build config service: {}", e);
            return;
        }
    };

    // 获取初始配置
    let config_resp = config_service
        .get_config(data_id.clone(), constants::DEFAULT_GROUP.to_string())
        .await;

    match config_resp {
        Ok(config_resp) => {
            info!(
                "Successfully get config from Nacos, data_id: {}, content length: {}",
                data_id,
                config_resp.content().len()
            );

            // 立即处理初始配置
            if let Some(routes) = parse_routes_from_config(config_resp.content()) {
                route_table.update_all_routes(routes.clone());
                super::update_routes_from_nacos(routes);
                info!("Updated routes from initial config");
            }

            // 添加配置监听 (在spawn中运行监听，这样config_service可以在函数完成时丢弃后继续运行)
            let config_service_clone = config_service.clone();
            let listener_clone = config_listener.clone();
            tokio::spawn(async move {
                let listen_result = config_service_clone
                    .add_listener(
                        data_id,
                        constants::DEFAULT_GROUP.to_string(),
                        listener_clone,
                    )
                    .await;

                match listen_result {
                    Ok(_) => {
                        info!("Listening to config changes successful");
                    }
                    Err(err) => {
                        error!("Listening to config changes failed: {:?}", err);
                    }
                }
            });
        }
        Err(e) => {
            error!("Failed to get config from Nacos: {:?}", e);
        }
    }

    // 初始化命名服务用于服务注册发现
    let client_props_naming = ClientProps::new()
        .server_addr(config.spring_config.cloud.nacos_config.server_addr.clone())
        .namespace(
            config
                .spring_config
                .cloud
                .nacos_config
                .discovery
                .namespace
                .clone(),
        )
        .auth_username(config.spring_config.cloud.nacos_config.username.clone())
        .auth_password(config.spring_config.cloud.nacos_config.password.clone());

    let naming_service = match NamingServiceBuilder::new(client_props_naming).build() {
        Ok(service) => service,
        Err(e) => {
            error!("Failed to build naming service: {}", e);
            return;
        }
    };

    // 注册服务实例到Nacos（可选，网关也可以注册自己）
    let service_instance = ServiceInstance {
        ip: get_local_ip(),
        port: config.server.port as i32,
        healthy: true,
        ..Default::default()
    };
    // 保存命名服务以供代理使用

    super::set_naming_service(naming_service.clone());

    let _register_result = naming_service
        .batch_register_instance(
            service_name,
            Some(constants::DEFAULT_GROUP.to_string()),
            vec![service_instance],
        )
        .await;

    info!("Nacos initialization completed");

    // Keep services alive
    std::mem::forget(naming_service);
    // config_service需要保持活跃以接收配置更新
    std::mem::forget(config_service);
}

// 解析配置中的路由信息
fn parse_routes_from_config(config_content: &str) -> Option<Vec<RouteConfig>> {
    // 尝试解析YAML内容
    match serde_yaml::from_str::<serde_json::Value>(config_content) {
        Ok(value) => {
            // 检查是否有路由配置
            if let Some(routes_array) = value.get("routes").and_then(|v| v.as_array()) {
                let mut routes = Vec::new();
                for route_value in routes_array {
                    if let Some(route) = convert_route_from_yaml(route_value) {
                        routes.push(route);
                    }
                }
                if !routes.is_empty() {
                    Some(routes)
                } else {
                    None
                }
            } else {
                None
            }
        }
        Err(e) => {
            error!("Failed to parse config content: {}", e);
            None
        }
    }
}

// 将YAML值转换为路由配置
fn convert_route_from_yaml(route_value: &serde_json::Value) -> Option<RouteConfig> {
    if let (Some(id), Some(uri)) = (
        route_value.get("id")?.as_str(),
        route_value.get("uri")?.as_str(),
    ) {
        // 解析predicates
        let predicates =
            if let Some(pred_array) = route_value.get("predicates").and_then(|v| v.as_array()) {
                pred_array
                    .iter()
                    .filter_map(|pred| {
                        if let Some(path) = pred.get("path").and_then(|v| v.as_str()) {
                            Some(PredicateConfig {
                                path: path.to_string(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            };

        // 解析filters
        let filters =
            if let Some(filter_array) = route_value.get("filters").and_then(|v| v.as_array()) {
                filter_array
                    .iter()
                    .map(|filter| {
                        let strip_prefix = filter
                            .get("strip_prefix")
                            .and_then(|v| v.as_u64())
                            .map(|v| v as u32);
                        FilterConfig { strip_prefix }
                    })
                    .collect()
            } else {
                Vec::new()
            };

        Some(RouteConfig {
            id: id.to_string(),
            uri: uri.to_string(),
            predicates,
            filters,
        })
    } else {
        None
    }
}

use nacos_sdk::api::naming::NamingServiceBuilder;

use tokio;

fn get_local_ip() -> String {
    match local_ipaddress::get() {
        Some(ip) => ip,
        None => {
            tracing::warn!("Failed to get local IP, using 127.0.0.1");
            "127.0.0.1".to_string()
        }
    }
}
