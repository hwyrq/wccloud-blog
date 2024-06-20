use sea_orm::{ConnectionTrait, EntityTrait, IntoActiveModel};

use crate::infrastructure::dao::entity::prelude::WebBlogLabel;
use crate::infrastructure::dao::entity::web_blog_label;

pub(crate) async fn save_blog_label( blog_id: i64, label_id: i64, x: &impl ConnectionTrait) {
    WebBlogLabel::insert(web_blog_label::Model {
        blog_id,
        label_id,
    }.into_active_model()).exec(x).await.unwrap();
}