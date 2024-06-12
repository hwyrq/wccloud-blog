//! author wcz

use std::any::type_name_of_val;

use redis::aio::MultiplexedConnection;
use redis::Client;
use redis_pool::connection::RedisPoolConnection;
use redis_pool::RedisPool;
use serde::Serialize;

use crate::infrastructure::config::get_config_value;

static  mut CON : Option<RedisPool<Client, MultiplexedConnection>> = None;

static mut KEY_PREFIX: String = String::new();

pub async  fn init_redis() {
    unsafe { KEY_PREFIX = get_config_value("spring.cache.redis.key-prefix"); }
    let host: String = get_config_value("spring.data.redis.host");
    let port: String = get_config_value("spring.data.redis.port");
    let password: String = get_config_value("spring.data.redis.password");
    let url = format!("redis://:{}@{}:{}/", password, host, port);
    log::info!("{}",url);

    let client = Client::open(url).unwrap();
    let pool = RedisPool::from(client);
    unsafe { CON = Some(pool) }
}

pub async   fn redis_master() -> RedisPoolConnection<MultiplexedConnection> {
    unsafe {
        CON.as_mut().unwrap().aquire().await.unwrap()
    }
}

pub fn cache_hash_key_field<T: ?Sized>(f: &T, n: &impl Serialize) -> (String, String) {
    unsafe { return (KEY_PREFIX.clone() + type_name_of_val(f), serde_json::to_string_pretty(n).unwrap()); }
}

pub fn cache_hash_key<T: ?Sized>(f: &T) -> String {
    unsafe { return KEY_PREFIX.clone() + type_name_of_val(f); }
}