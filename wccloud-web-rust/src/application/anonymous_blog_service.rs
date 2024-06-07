//! author wcz
use redis::{AsyncCommands, RedisResult};
use sea_orm::{EntityTrait, FromQueryResult, Statement};
use sea_orm::DatabaseBackend::MySql;
use tokio::join;
use xor_str::*;

use crate::controller::vo::web_blog_vo::{WebBlogOneRespVO, WebBlogPageReqVO, WebBlogPageRespVO};
use crate::controller::vo::web_label_vo::WebLabelResp;
use crate::infrastructure::config::redis_config::{cache_hash_key_field, redis_master};
use crate::infrastructure::config::sea_config::master;
use crate::infrastructure::dao::anonymous_blog_dao::{get_count, get_page_list};
use crate::infrastructure::dao::entity::{web_label, web_type};
use crate::infrastructure::dao::entity::prelude::WebBlog;
use crate::infrastructure::util::result::{PageResultVO, ResultVO, SuccessData};

pub async fn  page(req_vo: &WebBlogPageReqVO) -> ResultVO<PageResultVO<WebBlogPageRespVO>> {

    let (key,field) = cache_hash_key_field(&page, &req_vo);

    let x: RedisResult<PageResultVO<WebBlogPageRespVO>> = redis_master().await.hget(&key, &field).await;
    match x {
        Ok(x) => {return ResultVO::success(x)}
        Err(_) => {}
    }
    //异步执行，两个SQL一起执行，节省一半执行时间，nice
    let (list,count) = join!(get_page_list(&req_vo), get_count(&req_vo));

    let page_result_vo = PageResultVO::new(list, count);
    redis_master().await.hset::<&String,&String,String,i64>(&key, &field, serde_json::to_string_pretty(&page_result_vo).unwrap()).await.unwrap();
    return ResultVO::success(page_result_vo);
}

pub async fn one(blog_id:i64) -> ResultVO<WebBlogOneRespVO> {
    let option = WebBlog::find_by_id(blog_id).one(master()).await.unwrap().unwrap();
    let sql = "select b.* from web_blog_label a join web_label b on a.label_id = b.label_id where a.blog_id = ? ".to_string();
    let vec = web_label::Model::find_by_statement(Statement::from_sql_and_values(MySql, sql, [sea_orm::Value::BigInt(Some(blog_id))])).all(master()).await.unwrap();
    let mut vec1: Vec<String> = vec![];
    for value in vec {
        vec1.insert(0, value.label_name);
    }
    let sql = "select * from web_type where type_id = ?".to_string();
    let op = web_type::Model::find_by_statement(Statement::from_sql_and_values(MySql, sql, [sea_orm::Value::BigInt(Some(option.type_id))])).one(master()).await.unwrap();

    return ResultVO::success(WebBlogOneRespVO {
        blog_id: option.blog_id,
        title: option.title,
        img_url: option.img_url,
        summary: option.summary,
        type_name: op.unwrap().type_name,
        label_name: Some(vec1),
        level: option.level,
        enable_comment: option.enable_comment == 1,
        status: option.status,
        md: option.md,
        html: option.html,
    });
}

pub(crate) async fn level(level: i8)->ResultVO<Vec<WebBlogPageRespVO>> {

    let sql = xor!(r#"select a.*,"" type_name,"" label_name from web_blog a where level = ? and deleted = 0 order by blog_id desc "#) ;
    let vec = WebBlogPageRespVO::find_by_statement(Statement::from_sql_and_values(MySql, sql, [sea_orm::Value::from(level)])).all(master()).await.unwrap();
    return ResultVO::success(vec);
}

pub(crate) async fn label() -> ResultVO<Vec<WebLabelResp>> {
    let sql = xor!(r#"
    select distinct a.label_id,a.label_name
from web_label a 
join web_blog_label b on a.label_id = b.label_id
join web_blog c on c.blog_id = b.blog_id
where  a.deleted = 0 and c.deleted = 0 and c.status = 0
"#) ;
    let vec = WebLabelResp::find_by_statement(Statement::from_sql_and_values(MySql, sql, [])).all(master()).await.unwrap();
    return ResultVO::success(vec);
}