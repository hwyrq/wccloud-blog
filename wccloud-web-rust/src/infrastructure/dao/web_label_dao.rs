use chrono::Local;
use sea_orm::{ConnectionTrait, EntityTrait, InsertResult, IntoActiveModel, Statement, Value};
use sea_orm::DatabaseBackend::MySql;

use crate::infrastructure::config::snow_flake_config;
use crate::infrastructure::dao::entity::prelude::WebLabel;
use crate::infrastructure::dao::entity::web_label;
use crate::infrastructure::dao::entity::web_label::ActiveModel;

pub(crate) async fn delete_blog_label(blog_id: i64, x: &impl ConnectionTrait) {
    let sql = "delete from web_blog_label where blog_id = ? ".to_string();
    x.execute(Statement::from_sql_and_values(MySql, sql, [Value::BigInt(Some(blog_id))])).await.unwrap();
}

pub(crate) async fn get_list_by_list(sql: & String, vec: & Vec<Value>, x: &impl ConnectionTrait) -> Vec<web_label::Model> {
    let select_sql = format!("select*from web_label where label_name in({})", sql);
    let vec_web_label = WebLabel::find().from_raw_sql(Statement::from_sql_and_values(MySql, select_sql, vec.clone())).all(x).await.unwrap();
    vec_web_label
}
pub(crate) async fn recover_web_label(sql:& String, vec:& Vec<Value>, x: &impl ConnectionTrait) {
    let _ = x.execute(Statement::from_sql_and_values(
        MySql, format!("update web_label set deleted = 0 where label_name  in ({})", sql), vec.clone())).await;
}

pub(crate) async fn save_label(user_id: i64, label_name: &String, x: &impl ConnectionTrait) -> InsertResult<ActiveModel> {
    let result = WebLabel::insert(web_label::Model {
        label_id: snow_flake_config::next_id(),
        label_name: label_name.to_string(),
        create_user_id: user_id.clone(),
        create_time: Local::now().naive_local(),
        update_user_id: user_id.clone(),
        update_time: Local::now().naive_local(),
        deleted: 0,
    }.into_active_model()).exec(x).await.unwrap();
    result
}

pub(crate) async fn delete_unused_label(x:& impl ConnectionTrait) {
    x.execute(Statement::from_sql_and_values(
        MySql, "update web_label set deleted = 1 where label_id not in
        (select distinct label_id from web_blog_label a
        join web_blog b on a.blog_id = b.blog_id where b.deleted = 0 )", [])).await.expect("TODO: panic message")
    ;
}