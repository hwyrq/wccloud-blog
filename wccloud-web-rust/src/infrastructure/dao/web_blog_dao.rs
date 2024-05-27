//! author wcz
use lazy_static::lazy_static;
use sea_orm::{FromQueryResult, JsonValue, PaginatorTrait, Statement};
use sea_orm::DatabaseBackend::MySql;
use xor_str::decode;
use xor_str::encode;
use xor_str::xor;

use crate::controller::vo::web_blog_vo::{WebBlogPageReqVO, WebBlogPageRespVO};
use crate::infrastructure::config::sea_config::master;

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