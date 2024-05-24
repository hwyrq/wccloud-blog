//! author wcz

use std::any::type_name_of_val;

use redis::aio::MultiplexedConnection;
use redis::Client;
use redis_pool::connection::RedisPoolConnection;
use redis_pool::RedisPool;
use serde::Serialize;


use crate::infrastructure::util::constant::{REDIS_CACHE, REDIS_URL};

static  mut CON : Option<RedisPool<Client, MultiplexedConnection>> = None;

pub async  fn init_redis() {
    let client = redis::Client::open(REDIS_URL.to_string()).unwrap();
    let pool = RedisPool::from(client);
    unsafe { CON = Some(pool) }
}

pub async   fn redis_master() -> RedisPoolConnection<MultiplexedConnection> {
    unsafe {
        match &CON {
            Some(x) => x.aquire().await.unwrap(),
            None => todo!(),
        }
    }
}

pub fn cache_hash_key_field<T: ?Sized>(f: &T, n: &impl Serialize) -> (String, String) {
    return (REDIS_CACHE.clone() + type_name_of_val(f), serde_json::to_string_pretty(n).unwrap());
}

pub fn cache_hash_key<T: ?Sized>(f: &T) -> String {
    return REDIS_CACHE.clone() + type_name_of_val(f);
}