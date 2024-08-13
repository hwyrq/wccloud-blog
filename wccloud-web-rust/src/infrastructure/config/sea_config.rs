//! author wcz
use std::time::Duration;

use fastdate::DurationFrom;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::time::sleep;
use crate::infrastructure::config::get_config_value;

static mut DB:Option<DatabaseConnection> = None;

pub async  fn init_sea()  {
    let username: String = get_config_value("spring.datasource.dynamic.datasource.master.username");
    let password: String = get_config_value("spring.datasource.dynamic.datasource.master.password");
    let mysql_url: String = get_config_value("spring.datasource.dynamic.datasource.master.url");
    let mysql_url = mysql_url.replace("jdbc:p6spy:mysql://", "");
    let url = format!("mysql://{}:{}@{}",username, password,mysql_url);
    // log::info!("{}",url);

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_hour(1))
        .max_lifetime(Duration::from_hour(3))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let con = Database::connect(opt).await;
    match con.is_ok() {
        true => {
            unsafe { DB = Some(con.unwrap()); }
        }
        false => {
            sleep(Duration::from_secs(5)).await;
            Box::pin(init_sea()).await;
        }
    }
//entity-orm-cli generate entity -u mysql://root:9eD4PYotLh9f7u2KJnqxUCyX3kQZSXhn@home0122:3306/wccloud_blog -o src/infrastructure/dao/entity
    
}

pub fn master() ->  & 'static DatabaseConnection {
    unsafe { DB.as_mut().unwrap() }
}