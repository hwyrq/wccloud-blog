//! author wcz
use actix_web::{get, Responder, web};
use sea_orm::{FromQueryResult, JsonValue, Statement};
use sea_orm::DatabaseBackend::MySql;

use crate::infrastructure::config::sea_config::master;
use crate::infrastructure::util::result::{ResultVO, SuccessData};

#[get("/type/list/name")]
pub async fn list_name()-> impl Responder {
    let vec = JsonValue::find_by_statement(Statement::from_sql_and_values(
        MySql, "select type_name typeName from web_type where deleted = 0 ", [])).all(master()).await.unwrap();
    return ResultVO::success(vec);
}
pub fn web_type_controller(cfg: &mut web::ServiceConfig){
    cfg.service(list_name);
}