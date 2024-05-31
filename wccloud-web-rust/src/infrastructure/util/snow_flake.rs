//! 让生成的雪花ID类似于java mybatis plus 的雪花ID
use std::time::{Duration, UNIX_EPOCH};
use lazy_static::lazy_static;
use rand::distributions::Distribution;
use snowflake::SnowflakeIdGenerator;

static mut GENERATOR: Option<SnowflakeIdGenerator> = None;

lazy_static!{
  static ref _A:Option<SnowflakeIdGenerator>=  init();
}

fn init() -> Option<SnowflakeIdGenerator>{
    unsafe {
         GENERATOR = Option::from(SnowflakeIdGenerator
        ::with_epoch(rand::distributions::Uniform::new(1, 32).sample(&mut rand::thread_rng())
                     , rand::distributions::Uniform::new(1, 32).sample(&mut rand::thread_rng())
                     , UNIX_EPOCH + Duration::from_millis(1288834974657)));
        return GENERATOR;
    }
}


pub fn next_id() -> i64 {
    unsafe { GENERATOR.unwrap().real_time_generate() }
}