//! author wcz

use std::any::type_name_of_val;
use std::sync::OnceLock;
use std::time::Duration;

use redis::aio::MultiplexedConnection;
use redis::Client;
use redis_pool::connection::RedisPoolConnection;
use redis_pool::RedisPool;
use serde::Serialize;
use tokio::time::sleep;

use crate::infrastructure::config::get_config_value;

static CON: OnceLock<RedisPool<Client, MultiplexedConnection>> = OnceLock::new();
static KEY_PREFIX: OnceLock<String> = OnceLock::new();

pub async fn init_redis() {
    let prefix: String = get_config_value("spring.cache.redis.key-prefix");
    KEY_PREFIX.get_or_init(|| prefix);

    let host: String = get_config_value("spring.data.redis.host");
    let port: String = get_config_value("spring.data.redis.port");
    let password: String = get_config_value("spring.data.redis.password");
    let url = format!("redis://:{}@{}:{}/", password, host, port);

    let open = Client::open(url);
    match open {
        Ok(client) => {
            let pool = RedisPool::from(client);
            CON.get_or_init(|| pool);
        }
        Err(_) => {
            sleep(Duration::from_secs(5)).await;
            Box::pin(init_redis()).await;
        }
    }
}

pub async fn redis_master() -> RedisPoolConnection<MultiplexedConnection> {
    CON.get()
        .expect("Redis connection not initialized")
        .aquire()
        .await
        .expect("Failed to acquire Redis connection")
}

pub fn cache_hash_key_field<T: ?Sized>(f: &T, n: &impl Serialize) -> (String, String) {
    let prefix = KEY_PREFIX.get().expect("Key prefix not initialized");
    (
        prefix.clone() + type_name_of_val(f),
        serde_json::to_string_pretty(n).unwrap(),
    )
}

pub fn cache_hash_key<T: ?Sized>(f: &T) -> String {
    let prefix = KEY_PREFIX.get().expect("Key prefix not initialized");
    prefix.clone() + type_name_of_val(f)
}
