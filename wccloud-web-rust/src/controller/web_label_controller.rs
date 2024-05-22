//! author wcz
use actix_web::{get, Responder, web};
use sea_orm::{FromQueryResult, JsonValue, Statement};
use sea_orm::DatabaseBackend::MySql;

use crate::infrastructure::config::sea_config::master;
use crate::infrastructure::util::result::{ResultVO, SuccessData};

///@author wcz
#[get("/label/list/name")]
pub async fn list_name()-> impl Responder {
    let vec = JsonValue::find_by_statement(Statement::from_sql_and_values(
        MySql, "select label_name labelName from web_label where deleted = 0 ", [])).all(master()).await.unwrap();
    return ResultVO::success(vec);
}

pub fn web_label_controller(cfg: &mut web::ServiceConfig){
    cfg.service(list_name);
}

