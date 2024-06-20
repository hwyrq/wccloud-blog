use sea_orm::{FromQueryResult, JsonValue, Statement};
use sea_orm::DatabaseBackend::MySql;
use serde_json::Value;
use crate::infrastructure::config::sea_config::master;
use crate::infrastructure::util::result::{ResultVO, SuccessData};

pub async fn list_name() -> ResultVO<Vec<Value>> {
    let vec = JsonValue::find_by_statement(Statement::from_sql_and_values(
        MySql, "select type_name typeName from web_type where deleted = 0 ", [])).all(master()).await.unwrap();
    return ResultVO::success(vec);
}