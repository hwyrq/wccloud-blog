//! author wcz
use chrono::Local;
use lazy_static::lazy_static;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, FromQueryResult, IntoActiveModel, JsonValue, PaginatorTrait, Statement};
use sea_orm::DatabaseBackend::MySql;
use xor_str::decode;
use xor_str::encode;
use xor_str::xor;

use crate::controller::vo::web_blog_vo::{WebBlogPageReqVO, WebBlogPageRespVO, WebBlogSaveReqVO};
use crate::infrastructure::config::sea_config::master;
use crate::infrastructure::dao::entity::prelude::WebBlog;
use crate::infrastructure::dao::entity::web_blog;
use crate::infrastructure::dao::entity::web_blog::Column;

lazy_static! {
     static ref  PAGE_SQL:String = xor!("
    select a.blog_id, a.title,a.img_url, a.summary, a.type_id, a.level, a.enable_comment,
     a.status,a.update_time,b.type_name,c.label_name
from web_blog a
left join web_type b on a.type_id = b.type_id
left join (select blog_id, group_concat(label_name) label_name
           from web_blog_label a
                    join web_label b on a.label_id = b.label_id
           group by blog_id) c on c.blog_id = a.blog_id
where a.deleted = 0 order by a.blog_id desc
");
}
/// 获取分页查询数据列表
pub async fn get_page_list(arg: &WebBlogPageReqVO) -> Vec<WebBlogPageRespVO> {
    return WebBlogPageRespVO::find_by_statement(
        Statement::from_sql_and_values(MySql, PAGE_SQL.clone(), []))
        .paginate(master(), arg.page_size)
        .fetch_page(arg.page_num - 1)
        .await.unwrap();
}

///获取分页查询数据总数
pub async fn get_count(_arg: &WebBlogPageReqVO) -> u64 {
    return JsonValue::find_by_statement(
        Statement::from_sql_and_values(MySql, PAGE_SQL.clone(), [])).count(master()).await.unwrap();
}

pub async fn save_or_update_blog(arg: WebBlogSaveReqVO, type_id: &mut i64, blog_id: &mut i64, user_id: i64, x: &impl ConnectionTrait) {
    let mut model = web_blog::Model {
        blog_id: *blog_id,
        title: arg.title.clone(),
        summary: (arg.summary).parse().unwrap(),
        type_id: *type_id,
        level: arg.level,
        enable_comment: match arg.enable_comment {
            true => { 1 }
            false => { 0 }
        },
        status: arg.status,
        html: (arg.html).parse().unwrap(),
        md: (arg.md).parse().unwrap(),
        img_url: match &arg.img_url {
            None => { "".to_string() }
            Some(v) => { v.to_string() }
        },
        create_user_id: user_id.clone(),
        create_time: Local::now().naive_local(),
        update_user_id: user_id.clone(),
        update_time: Local::now().naive_local(),
        deleted: 0,
    }.into_active_model();

    match arg.clone().blog_id {
        None => { WebBlog::insert(model).exec(x).await.unwrap(); }
        Some(_i) => {
            model.reset(Column::BlogId);
            model.reset(Column::Title);
            model.reset(Column::Summary);
            model.reset(Column::TypeId);
            model.reset(Column::Level);
            model.reset(Column::EnableComment);
            model.reset(Column::Status);
            model.reset(Column::Html);
            model.reset(Column::Md);
            model.reset(Column::ImgUrl);
            // model.reset(Column::CreateUserId);
            // model.reset(Column::CreateTime);
            model.reset(Column::UpdateUserId);
            model.reset(Column::UpdateTime);
            model.reset(Column::Deleted);

            model.update(x).await.unwrap();
        }
    }
}