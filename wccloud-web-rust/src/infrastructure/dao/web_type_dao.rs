use chrono::Local;
use sea_orm::{ConnectionTrait, EntityTrait, IntoActiveModel, Statement, Value};
use sea_orm::DatabaseBackend::MySql;
use sea_orm::prelude::DateTime;

use crate::controller::vo::web_blog_vo::WebBlogSaveReqVO;
use crate::infrastructure::dao::entity::prelude::WebType;
use crate::infrastructure::dao::entity::web_type::Model;

pub(crate) async fn get_web_type(value: &Value, x: & impl ConnectionTrait) -> Option<Model>  {
    WebType::find().from_raw_sql(Statement::from_sql_and_values(
        MySql, "select*from web_type where type_name = ? ",
        [value.clone()])).one(x).await.unwrap()
}

pub(crate) async fn save_web_type(arg: &WebBlogSaveReqVO, type_id:  i64, user_id: i64, x: &impl ConnectionTrait) {
    let active_model = Model {
        type_id,
        type_name: arg.type_name.to_string(),
        create_user_id: user_id.clone(),
        create_time: Local::now().naive_local(),
        update_user_id: user_id.clone(),
        update_time: DateTime::default(),
        deleted: 0,
    }.into_active_model();

    WebType::insert(active_model).exec(x).await.unwrap();
}

pub(crate) async fn recover_web_type(value: Value, x: &impl ConnectionTrait) {
    let _ = x.execute(Statement::from_sql_and_values(
        MySql, "update web_type set deleted = 0 where type_id = ? ", [value])).await;
}

pub(crate) async fn delete_unused_type(x: &impl ConnectionTrait) {
    x.execute(Statement::from_sql_and_values(
        MySql, "update web_type set deleted = 1 where type_id not in \
        (select distinct type_id from web_blog where deleted = 0)", [])).await.unwrap()
    ;
}