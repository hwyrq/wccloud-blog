//! author wcz
use std::sync::Arc;

use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, };
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{NamingChangeEvent, NamingEventListener, NamingServiceBuilder, ServiceInstance};
use nacos_sdk::api::naming::NamingService;
use nacos_sdk::api::props::ClientProps;
#[allow(dead_code)]
struct MyConfigChangeListener;

impl ConfigChangeListener for MyConfigChangeListener {
    fn notify(&self, config_resp: ConfigResponse) {
        log::info!("listener the config = {:?}",config_resp);
    }
}

struct MyInstanceChangeListener;

impl NamingEventListener for MyInstanceChangeListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        log::info!("subscriber notify event = {:?}",event);
    }
}
  pub async  fn init_nacos() {
    let client_props = ClientProps::new()
        .server_addr("home0122:8848")
        .namespace("wccloud-dev")
        .app_name("simple_app")
        .auth_username("nacos")
        .auth_password("nacos");
      log::info!("{:?}",&client_props);

      //config

/*    let config_server = ConfigServiceBuilder::new(client_props.clone()).enable_auth_plugin_http().build().expect("info");

    let config_resp = config_server.get_config("todo-data-id".to_string(), "todo-group".to_string());
    match config_resp {
        Ok(config_resp) => {
            log::info!("get the config {}",config_resp)
        }
        Err(err) => {
            tracing::error!("get the config {:?}",err)
        }
    }

    let _listen = config_server.add_listener("todo-data-id".to_string(), "todo-group".to_string(), Arc::new(MyConfigChangeListener {}));
    match _listen {
        Ok(_) => {
            log::info!("listening the config success")
        }
        Err(err) => {
            tracing::error!("listening config error{:?}",err)
        }
    }*/

    let naming_server = NamingServiceBuilder::new(client_props).enable_auth_plugin_http().build().expect("ddddd");

    let subscriber = Arc::new(MyInstanceChangeListener);
    let _subscriber_ret = naming_server.subscribe("wccloud-web-rust".to_string(), Some(constants::DEFAULT_GROUP.to_string()), Vec::default(), subscriber);
    let server_instance = ServiceInstance {
        ip: "127.0.0.1".to_string(),
        port: 8085,
        ..Default::default()
    };
    let _register_instance_ret = naming_server.batch_register_instance("wccloud-web-rust".to_string(), Some(constants::DEFAULT_GROUP.to_string()), vec![server_instance]);

}