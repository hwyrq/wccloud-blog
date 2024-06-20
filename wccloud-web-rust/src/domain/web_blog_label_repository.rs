use sea_orm::ConnectionTrait;
use crate::infrastructure::dao::entity::web_label;
use crate::infrastructure::dao::web_blog_label_dao;

pub(crate) async fn batch_save_blog_label( blog_id: i64, vec_web_label: &Vec<web_label::Model>, x: &impl ConnectionTrait) {
    for model in vec_web_label {
        //新增关系
        web_blog_label_dao::save_blog_label(blog_id, model.label_id, x).await;
    }
}

