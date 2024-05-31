//! author wcz
use std::time::Duration;
use fastdate::DurationFrom;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::infrastructure::util::constant::MYSQL_URL;

static mut DB:DatabaseConnection = DatabaseConnection::Disconnected;

pub async  fn init_sea()  {
    let mut opt = ConnectOptions::new(MYSQL_URL.to_string());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_hour(1))
        .max_lifetime(Duration::from_hour(3))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
//entity-orm-cli generate entity -u mysql://root:9eD4PYotLh9f7u2KJnqxUCyX3kQZSXhn@home0122:3306/wccloud_blog -o src/infrastructure/dao/entity
    unsafe { DB = Database::connect(opt).await.unwrap(); }
}

pub fn master() ->  & 'static DatabaseConnection {
    unsafe { &DB }
}