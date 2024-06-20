use sea_orm::ConnectionTrait;

use crate::controller::vo::web_blog_vo::WebBlogSaveReqVO;
use crate::domain::web_blog_label_repository;
use crate::infrastructure::dao::{web_blog_label_dao, web_label_dao};

pub(crate) async fn process_label(arg: &&WebBlogSaveReqVO, blog_id: i64, user_id: i64, x: &impl ConnectionTrait) {
    //先删除关系
    web_label_dao::delete_blog_label(blog_id,x).await;

    let mut sql = "".to_string();
    let mut vec = vec![];
    for str in &arg.label_name {
        sql = sql + "?,";
        vec.insert(0, sea_orm::Value::from(str));
    }
    sql.remove(sql.len() - 1);
    let vec_web_label = web_label_dao::get_list_by_list(&sql, &vec,x).await;
    //并吧deleted设置为0
    web_label_dao::recover_web_label(&sql, &vec,x).await;
    let mut contains_name = vec![];
    //先把不存在的找出来，并新建
    for model in &vec_web_label {
        contains_name.insert(0, &model.label_name);
    }
    for label_name in &arg.label_name {
        if !contains_name.contains(&label_name) {
            //不包含，则新增
            let result = web_label_dao::save_label(user_id, label_name,x).await;
            //新增关系
            web_blog_label_dao::save_blog_label(blog_id, result.last_insert_id,x).await;
        };
    }
    //已包含，则新增关系
    web_blog_label_repository::batch_save_blog_label(blog_id, &vec_web_label,x).await;
}