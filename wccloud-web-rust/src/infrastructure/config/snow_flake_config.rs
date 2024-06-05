//! 让生成的雪花ID类似于java mybatis plus 的雪花ID
use std::time::{Duration, UNIX_EPOCH};

use rand::distributions::Distribution;
use snowflake::SnowflakeIdGenerator;

static mut GENERATOR: Option<SnowflakeIdGenerator> = None;

pub async fn init_snow_flake(){
    unsafe {
        let machine_id = rand::distributions::Uniform::new(1, 32).sample(&mut rand::thread_rng());
        let node_id = rand::distributions::Uniform::new(1, 32).sample(&mut rand::thread_rng());
        let epoch = UNIX_EPOCH + Duration::from_millis(1288834974657);
        log::info!("init_snow_flake:machine_id:{},node_id:{}",machine_id,node_id);
         GENERATOR = Some(SnowflakeIdGenerator::with_epoch(machine_id,node_id,epoch));
    }
}


pub fn next_id() -> i64 {
    unsafe { GENERATOR.as_mut().unwrap().real_time_generate() }
}

#[tokio::test]
pub async  fn test() {
    init_snow_flake().await;
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
    println!("{}", next_id());
}

