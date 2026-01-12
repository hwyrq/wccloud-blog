//! 让生成的雪花ID类似于java mybatis plus 的雪花ID
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, UNIX_EPOCH};

use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::thread_rng;
use snowflake::SnowflakeIdGenerator;

static GENERATOR: OnceLock<Mutex<SnowflakeIdGenerator>> = OnceLock::new();

pub async fn init_snow_flake() {
    let machine_id = Uniform::new(1, 32).sample(&mut thread_rng());
    let node_id = Uniform::new(1, 32).sample(&mut thread_rng());
    let epoch = UNIX_EPOCH + Duration::from_millis(1288834974657);
    log::info!(
        "init_snow_flake:machine_id:{},node_id:{}",
        machine_id,
        node_id
    );
    GENERATOR
        .get_or_init(|| Mutex::new(SnowflakeIdGenerator::with_epoch(machine_id, node_id, epoch)));
}

pub fn next_id() -> i64 {
    let generator = GENERATOR
        .get()
        .expect("Snowflake generator not initialized");
    let mut gen = generator.lock().unwrap();
    gen.real_time_generate()
}

#[tokio::test]
pub async fn test() {
    init_snow_flake().await;
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
}
