//! author wcz
use actix_web::HttpRequest;
use redis::{AsyncCommands, RedisResult};
use sea_orm::{ ConnectionTrait, EntityTrait, FromQueryResult, Statement, TransactionTrait};
use sea_orm::DatabaseBackend::MySql;
use tokio::join;

use crate::controller::vo::web_blog_vo::{WebBlogOneRespVO, WebBlogPageReqVO, WebBlogPageRespVO, WebBlogSaveReqVO};
use crate::domain::{web_label_repository, web_type_repository};
use crate::infrastructure::config::redis_config::{cache_hash_key, cache_hash_key_field, redis_master};
use crate::infrastructure::config::sea_config::master;
use crate::infrastructure::config::snow_flake_config;
use crate::infrastructure::dao::{web_blog_dao, web_label_dao, web_type_dao};
use crate::infrastructure::dao::entity::{web_label, web_type};
use crate::infrastructure::dao::entity::prelude::WebBlog;
use crate::infrastructure::dao::web_blog_dao::{get_count, get_page_list};
use crate::infrastructure::util::result::{PageResultVO, ResultVO, SuccessData};

pub async fn page(req_vo: &WebBlogPageReqVO) -> ResultVO<PageResultVO<WebBlogPageRespVO>> {
    let (key, field) = cache_hash_key_field(&page, &req_vo);

    let x: RedisResult<PageResultVO<WebBlogPageRespVO>> = redis_master().await.hget(&key, &field).await;
    match x {
        Ok(x) => { return ResultVO::success(x); }
        Err(_) => {}
    }
    //异步执行，两个SQL一起执行，节省一半执行时间，nice
    let (list, count) = join!(get_page_list(&req_vo), get_count(&req_vo));

    let page_result_vo = PageResultVO::new(list, count);
    redis_master().await.hset::<&String, &String, String, i64>(&key, &field, serde_json::to_string_pretty(&page_result_vo).unwrap()).await.unwrap();
    return ResultVO::success(page_result_vo);
}

pub async fn save(arg: &WebBlogSaveReqVO, http_request: &HttpRequest) -> ResultVO<bool> {
    let mut type_id = snow_flake_config::next_id();
    let mut blog_id = snow_flake_config::next_id();
    let user_id = redis_master().await.get::<&str, i64>(
        &*("accessToken:".to_string() + http_request.headers().get("token").unwrap().to_str().unwrap())
    ).await.unwrap();
    //获取用户信息
    //根据前端传参，判断是新增还是更新
    match arg.blog_id {
        None => {}
        Some(_) => {
            blog_id = arg.blog_id.as_ref().unwrap().parse().unwrap();
        }
    }
    //开启事务
    //rust如果也有类似springboot的框架就好了，就不用手写事务了
    let transaction = master().begin().await.unwrap();

    //异步的魅力，
    join!(
       //判断并新增type;rust牛逼呀，type_id在这个方法内变更后，那么外部方法体的值就变更了，java就不行，return恶心人，或者得包装成bean
    web_type_repository::process_type(arg,&mut type_id, user_id,&transaction),
    //判断并新增label新增关系
    web_label_repository::process_label(&arg, blog_id, user_id,&transaction)
    );
    //保存或更新博客
    web_blog_dao::save_or_update_blog(arg.clone(), &mut type_id, &mut blog_id, user_id, &transaction).await;
    join!(
         //更新下type和label表，自动删除掉未引用的，就懒得做这两个的管理了，
    //not in ，嗯，别喷，后期给type和label正名，做可视化管理
        web_type_dao::delete_unused_type(&transaction),
        web_label_dao::delete_unused_label(&transaction),
    );
    transaction.commit().await.unwrap();
    let mut connection = redis_master().await;
    connection.del::<String, i64>(cache_hash_key(&page)).await.unwrap();
    connection.del::<String, i64>(cache_hash_key(&crate::application::anonymous_blog_service::page)).await.unwrap();

    return ResultVO::success(true);
}


pub async fn one(blog_id: i64) -> ResultVO<WebBlogOneRespVO> {
    let (key,field) = cache_hash_key_field(&one, &blog_id);

    let x: RedisResult<PageResultVO<WebBlogOneRespVO>> = redis_master().await.hget(&key, &field).await;
    match x {
        Ok(mut x) => {
            log::info!("缓存读取......");
            let r = x.list.pop().unwrap();
            return ResultVO::success(r)
        }
        Err(_) => {}
    }
    let option = WebBlog::find_by_id(blog_id).one(master()).await.unwrap().unwrap();
    let sql = "select b.* from web_blog_label a join web_label b on a.label_id = b.label_id where a.blog_id = ? ".to_string();
    let vec = web_label::Model::find_by_statement(Statement::from_sql_and_values(MySql, sql, [sea_orm::Value::BigInt(Some(blog_id))])).all(master()).await.unwrap();
    let mut vec1: Vec<String> = vec![];
    for value in vec {
        vec1.insert(0, value.label_name);
    }
    let sql = "select * from web_type where type_id = ?".to_string();
    let op = web_type::Model::find_by_statement(Statement::from_sql_and_values(MySql, sql, [sea_orm::Value::BigInt(Some(option.type_id))])).one(master()).await.unwrap();
    let vo = WebBlogOneRespVO {
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
    };
    redis_master().await.hset::<&String,&String,String,i64>(&key, &field, serde_json::to_string_pretty(&PageResultVO::new(vec![&vo], 0)).unwrap()).await.unwrap();
    
    return ResultVO::success(vo);
}

pub async fn delete(blog_id: i64) -> ResultVO<bool> {
    let _ = master().execute(Statement::from_sql_and_values(
        MySql, "update web_blog set deleted = 1 where blog_id = ? ",
        [sea_orm::Value::BigInt(Some(blog_id))])).await.unwrap();
    redis_master().await.del::<String, i64>(cache_hash_key(&page)).await.unwrap();
    redis_master().await.del::<String, i64>(cache_hash_key(&crate::application::anonymous_blog_service::page)).await.unwrap();

    return ResultVO::success(true);
}