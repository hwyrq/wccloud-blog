//! author wcz
use std::sync::Arc;

use config::{Config, FileFormat};
use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, ConfigService, ConfigServiceBuilder};
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{NamingChangeEvent, NamingEventListener, NamingServiceBuilder, ServiceInstance};
use nacos_sdk::api::naming::NamingService;
use nacos_sdk::api::props::ClientProps;

use crate::infrastructure::config::{CONF, get_config_value};
use crate::infrastructure::config::redis_config::init_redis;
use crate::infrastructure::config::sea_config::init_sea;

#[allow(dead_code)]
struct MyConfigChangeListener;

impl ConfigChangeListener for MyConfigChangeListener {
      fn notify(&self, config_resp: ConfigResponse) {
        log::info!("refresh config ");
        //获取到最新的配置文件后，需要重新配置系统
        let  app_yml = Config::builder()
            .add_source(config::File::with_name("application.yml"))
            .add_source(config::File::from_str(config_resp.content(),FileFormat::Yaml))
            .build().unwrap();
        unsafe { CONF = Some(app_yml); }
          tokio::spawn(init_sea());
          tokio::spawn(init_redis());
    }
}

struct MyInstanceChangeListener;

impl NamingEventListener for MyInstanceChangeListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        log::info!("subscriber notify event = {:?}",event);
    }
}

pub async fn init_nacos() {
    let server_addr = get_config_value::<String>("spring.cloud.nacos.server-addr");
    let namespace =get_config_value::<String>("spring.cloud.nacos.discovery.namespace");
    /*let auth_username = get_config_value::<String>("spring.cloud.nacos.username");
    let auth_password = get_config_value::<String>("spring.cloud.nacos.password");*/
    let service_name = get_config_value::<String>("spring.application.name");
    let config_name = get_config_value::<String>("spring.cloud.nacos.config.name");
    let file_extension = get_config_value::<String>("spring.cloud.nacos.config.file-extension");
    let data_id =  config_name + "." + &*file_extension;
    let arc = Arc::new(MyConfigChangeListener {});
    let client_props = ClientProps::new()
        .server_addr(server_addr)
        .namespace(namespace)
        .app_name(service_name.clone())
        /*.auth_username(auth_username)
        .auth_password(auth_password)*/;
    log::info!("{:?}",&client_props);
    //config
    let config_server = ConfigServiceBuilder::new(
        client_props.clone()
    )/*.enable_auth_plugin_http()*/.build().unwrap();

    let config_resp = config_server.get_config(
        data_id.clone(),
        constants::DEFAULT_GROUP.to_string(),
    );
    match config_resp {
        Ok(config_resp) => {
            arc.notify(config_resp);
            listener(service_name, data_id, arc, client_props, config_server);
        }
        Err(err) => {
            tracing::error!("get the config {:?}",err);
        }
    }



}

fn listener(service_name: String, data_id: String, arc: Arc<MyConfigChangeListener>, client_props: ClientProps, config_server: impl ConfigService + Sized) {
    let _listen = config_server.add_listener(
        data_id,
        constants::DEFAULT_GROUP.into(),
        arc,
    );
    match _listen {
        Ok(_) => {
            log::info!("listening the config success")
        }
        Err(err) => {
            tracing::error!("listening config error{:?}",err)
        }
    }
    let naming_server = NamingServiceBuilder::new(client_props)/*.enable_auth_plugin_http()*/.build().unwrap();
    let subscriber = Arc::new(MyInstanceChangeListener);
    let _subscriber_ret = naming_server.subscribe(
        service_name.clone(),
        Some(constants::DEFAULT_GROUP.to_string()),
        Vec::default(),
        subscriber
    );
    let server_instance = ServiceInstance {
        ip: get_local_info::get_pc_ipv4(),
        port: get_config_value::<i32>("server.port"),
        ..Default::default()
    };
    let _register_instance_ret = naming_server.batch_register_instance(
        service_name,
        Some(constants::DEFAULT_GROUP.to_string()),
        vec![server_instance]
    );
}