pub mod nacos_config;

use config::{Config, File};

use crate::domain::models::{FilterConfig, PredicateConfig, RouteConfig};

#[derive(Debug, serde::Deserialize)]
pub struct GatewayConfig {
    pub server: ServerConfig,
    #[serde(rename = "spring")]
    pub spring_config: SpringConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct SpringConfig {
    pub application: ApplicationConfig,
    pub cloud: CloudConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CloudConfig {
    #[serde(rename = "nacos")]
    pub nacos_config: NacosSpringConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct NacosSpringConfig {
    #[serde(rename = "server-addr")]
    pub server_addr: String,
    pub username: String,
    pub password: String,
    pub discovery: DiscoveryConfig,
    pub config: ConfigNamespaceConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct DiscoveryConfig {
    pub namespace: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ConfigNamespaceConfig {
    pub namespace: String,
    pub name: String,
    #[serde(rename = "file-extension")]
    pub file_extension: String,
}

// Internal configuration for our use
#[derive(Debug)]
pub struct NacosConfig {
    pub server_addr: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub config_name: String,
    pub file_extension: String,
}

impl From<&NacosSpringConfig> for NacosConfig {
    fn from(nacos: &NacosSpringConfig) -> Self {
        NacosConfig {
            server_addr: nacos.server_addr.clone(),
            username: nacos.username.clone(),
            password: nacos.password.clone(),
            namespace: nacos.discovery.namespace.clone(),
            config_name: nacos.config.name.clone(),
            file_extension: nacos.config.file_extension.clone(),
        }
    }
}

impl SpringConfig {
    pub fn get_nacos_config(&self) -> NacosConfig {
        (&self.cloud.nacos_config).into()
    }
}

// For runtime modification of routes
use once_cell::sync::Lazy;
use std::sync::RwLock;

static G_CONFIG: Lazy<RwLock<GatewayConfigWithRoutes>> = Lazy::new(|| {
    RwLock::new(GatewayConfigWithRoutes {
        server: ServerConfig { port: 8081 },
        spring_config: SpringConfig {
            application: ApplicationConfig {
                name: "wccloud-gateway-rust".to_string(),
            },
            cloud: CloudConfig {
                nacos_config: NacosSpringConfig {
                    server_addr: "10.96.3.1:8848".to_string(),
                    username: "nacos".to_string(),
                    password: "nacos".to_string(),
                    discovery: DiscoveryConfig {
                        namespace: "wccloud-dev".to_string(),
                    },
                    config: ConfigNamespaceConfig {
                        namespace: "wccloud-dev".to_string(),
                        name: "wccloud-shared".to_string(),
                        file_extension: "yaml".to_string(),
                    },
                },
            },
        },
        routes: vec![
            RouteConfig {
                id: "auth".to_string(),
                uri: "lb://wccloud-auth-server".to_string(),
                predicates: vec![PredicateConfig {
                    path: "/wccloud-auth-server/**".to_string(),
                }],
                filters: vec![FilterConfig {
                    strip_prefix: Some(1),
                }],
            },
            RouteConfig {
                id: "web".to_string(),
                uri: "lb://wccloud-web-server".to_string(),
                predicates: vec![PredicateConfig {
                    path: "/wccloud-web-server/**".to_string(),
                }],
                filters: vec![FilterConfig {
                    strip_prefix: Some(1),
                }],
            },
            RouteConfig {
                id: "admin".to_string(),
                uri: "lb://wccloud-admin-server".to_string(),
                predicates: vec![PredicateConfig {
                    path: "/wccloud-admin-server/**".to_string(),
                }],
                filters: vec![FilterConfig {
                    strip_prefix: Some(1),
                }],
            },
            RouteConfig {
                id: "web-rust".to_string(),
                uri: "lb://wccloud-web-rust".to_string(),
                predicates: vec![PredicateConfig {
                    path: "/wccloud-web-rust/**".to_string(),
                }],
                filters: vec![FilterConfig {
                    strip_prefix: Some(1),
                }],
            },
        ],
    })
});

// This will contain both the config from file and additional routes
#[derive(Debug)]
pub struct GatewayConfigWithRoutes {
    pub server: ServerConfig,
    pub spring_config: SpringConfig,
    pub routes: Vec<RouteConfig>,
}
impl GatewayConfigWithRoutes {
    pub fn get_nacos_config(&self) -> NacosConfig {
        (&self.spring_config.cloud.nacos_config).into()
    }
}

// 从 config 模块直接获取服务端口
pub fn get_server_port() -> u16 {
    let config = get_config();
    config.server.port
}

// 从 config 模块直接获取应用名称
pub fn get_application_name() -> String {
    let config = get_config();
    config.spring_config.application.name.clone()
}

pub fn get_config() -> std::sync::RwLockReadGuard<'static, GatewayConfigWithRoutes> {
    G_CONFIG.read().unwrap()
}

pub fn get_config_mut() -> std::sync::RwLockWriteGuard<'static, GatewayConfigWithRoutes> {
    G_CONFIG.write().unwrap()
}

pub fn init_config_file() {
    let config_result = Config::builder()
        .add_source(File::with_name("application").required(false))
        .add_source(File::with_name("application-pro").required(false))
        .build();

    match config_result {
        Ok(config) => {
            let config_from_file: GatewayConfig = config
                .try_deserialize()
                .expect("Failed to deserialize config");

            // Update the shared config with the loaded values
            {
                let mut config_guard = get_config_mut();
                config_guard.server = config_from_file.server;
                config_guard.spring_config = config_from_file.spring_config;
            }
        }

        Err(e) => {
            println!("Config file not found, using default settings: {}", e);
        }
    }
}

// 更新共享配置中的路由
pub fn update_routes_from_nacos(routes: Vec<RouteConfig>) {
    let mut config_guard = get_config_mut();
    config_guard.routes = routes;

    println!(
        "Updated routes from Nacos, total routes: {}",
        config_guard.routes.len()
    );
}

use std::sync::Mutex;

static SHARED_NAMING_SERVICE: Lazy<Mutex<Option<nacos_sdk::api::naming::NamingService>>> =
    Lazy::new(|| Mutex::new(None));

pub fn set_naming_service(naming_service: nacos_sdk::api::naming::NamingService) {
    if let Ok(mut guard) = SHARED_NAMING_SERVICE.lock() {
        *guard = Some(naming_service);
    }
}

pub fn get_naming_service() -> Option<nacos_sdk::api::naming::NamingService> {
    if let Ok(guard) = SHARED_NAMING_SERVICE.lock() {
        guard.clone()
    } else {
        None
    }
}
