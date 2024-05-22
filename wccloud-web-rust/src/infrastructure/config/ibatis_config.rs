/*
//! author wcz
use lazy_static::lazy_static;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use xor_str::xor;
use xor_str::encode;
use xor_str::decode;
lazy_static! {
  static ref  BT: RBatis = RBatis::new();
}

pub async fn init_ibatis() {
    let d = MysqlDriver {};
    let rb = BT.link(d, &*xor!("mysql://root:9eD4PYotLh9f7u2KJnqxUCyX3kQZSXhn@home0122:3306/wccloud_blog")).await;
    log::info!("ibatis init : {}", rb.is_ok());
}

pub fn master() -> &'static RBatis {
    &BT
}*/