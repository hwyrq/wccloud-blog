//! author wcz
use redis::{AsyncCommands, RedisResult};
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, FromQueryResult, IntoActiveModel, Statement};
use sea_orm::DatabaseBackend::MySql;
use sea_orm::prelude::DateTime;
use tokio::join;

use crate::controller::vo::web_blog_vo::{WebBlogOneRespVO, WebBlogPageReqVO, WebBlogPageRespVO, WebBlogSaveReqVO};
use crate::infrastructure::config::redis_config::{cache_hash_key, cache_hash_key_field, redis_master};
use crate::infrastructure::config::sea_config::master;
use crate::infrastructure::dao::entity::{web_blog, web_blog_label, web_label, web_type};
use crate::infrastructure::dao::entity::prelude::{WebBlog, WebBlogLabel, WebLabel, WebType};
use crate::infrastructure::dao::web_blog_dao::{get_count, get_page_list};
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

pub async fn save(arg: &WebBlogSaveReqVO) ->ResultVO<bool>{
    let mut type_id = spaceflake::generate(spaceflake::GeneratorSettings::default()).unwrap().id as i64;
    let mut blog_id = spaceflake::generate(spaceflake::GeneratorSettings::default()).unwrap().id as i64;

    match arg.clone().blog_id {
        None => {}
        Some(_) => {//删除
            blog_id = arg.clone().blog_id.unwrap().parse().unwrap();
            let sql = "delete from web_blog_label where blog_id = ? ".to_string();
            master().execute(Statement::from_sql_and_values(MySql, sql, [sea_orm::Value::BigInt(Some(blog_id))])).await.unwrap();
        }
    }
    //判断当前type是否存在
    let value = sea_orm::Value::from(arg.type_name.to_string());
    let option_web_tpe = WebType::find().from_raw_sql(Statement::from_sql_and_values(
        MySql, "select*from web_type where type_name = ? ",
        [value.clone()])).one(master()).await.unwrap();
    match option_web_tpe {
        None => {
            let active_model = web_type::Model {
                type_id,
                type_name: arg.type_name.to_string(),
                create_user_id: 1,
                create_time: DateTime::default(),
                update_user_id: 1,
                update_time: DateTime::default(),
                deleted: 0,
            }.into_active_model();

            WebType::insert(active_model).exec(master()).await.unwrap();
        }
        Some(model) => {
            type_id = model.type_id;
            //并吧deleted设置为0
            let _ = master().execute(Statement::from_sql_and_values(
                MySql, "update web_type set deleted = 0 where type_id = ? ", [value])).await;

        }
    }
    //判断当前lebel是否存在
    let mut sql = "".to_string();
    let mut vec = vec![];
    for str in &arg.label_name {
        sql = sql + "?,";
        vec.insert(0, sea_orm::Value::from(str));
    }
    sql.remove(sql.len() - 1);
    let select_sql = format!("select*from web_label where label_name in({})", sql);
    let vec_web_label = WebLabel::find().from_raw_sql(Statement::from_sql_and_values(MySql, select_sql, vec.clone())).all(master()).await.unwrap();
    //并吧deleted设置为0
    let _ = master().execute(Statement::from_sql_and_values(
        MySql, format!("update web_label set deleted = 0 where label_name  in ({})", sql), vec)).await;
    let mut contains_name = vec![];
    //先把不存在的找出来，并新建
    for model in &vec_web_label {
        contains_name.insert(0, &model.label_name);
    }
    for label_name in &arg.label_name {
        if !contains_name.contains(&label_name) {
            //不包含，则新增
            let result = WebLabel::insert(web_label::Model {
                label_id: spaceflake::generate(spaceflake::GeneratorSettings::default()).unwrap().id as i64,
                label_name: label_name.to_string(),
                create_user_id: 0,
                create_time: DateTime::default(),
                update_user_id: 0,
                update_time: DateTime::default(),
                deleted: 0,
            }.into_active_model()).exec(master()).await.unwrap();
            //新增关系
            WebBlogLabel::insert(web_blog_label::Model {
                blog_id,
                label_id: result.last_insert_id,
            }.into_active_model()).exec(master()).await.unwrap();
        };
    }
    //已包含，则新增关系
    for model in &vec_web_label {
        //新增关系
        WebBlogLabel::insert(web_blog_label::Model {
            blog_id,
            label_id: model.label_id,
        }.into_active_model()).exec(master()).await.unwrap();
    }


    let model = web_blog::Model {
        blog_id,
        title: arg.title.clone(),
        summary: (arg.summary).parse().unwrap(),
        type_id,
        level: arg.level,
        enable_comment: match arg.enable_comment {
            true => { 1 }
            false => { 0 }
        },
        status: arg.status,
        html: (arg.html).parse().unwrap(),
        md: (arg.md).parse().unwrap(),
        create_user_id: 0,
        create_time: DateTime::default(),
        update_user_id: 0,
        update_time: DateTime::default(),
        deleted: 0,
    }.into_active_model();

    match arg.clone().blog_id {
        None => { WebBlog::insert(model).exec(master()).await.unwrap(); }
        Some(_i) => {
            model.reset_all().update(master()).await.unwrap();
        }
    }
    //更新下type和label表，自动删除掉未引用的，就懒得做这两个的管理了，
    //not in ，嗯，别喷，后期给type和label正名，做可视化管理
    async fn update1() {
        master().execute(Statement::from_sql_and_values(
            MySql, "update web_type set deleted = 1 where type_id not in \
        (select distinct type_id from web_blog where deleted = 0)", [])).await.expect("TODO: panic message")
        ;
    }
    async fn update2() {
        master().execute(Statement::from_sql_and_values(
            MySql, "update web_label set deleted = 1 where label_id not in
        (select distinct label_id from web_blog_label a
        join web_blog b on a.blog_id = b.blog_id where b.deleted = 0 )", [])).await.expect("TODO: panic message")
        ;
    }
    join!(update1(),update2());

    redis_master().await.del::<String,i64>(cache_hash_key(&page)).await.unwrap();
    redis_master().await.del::<String,i64>(cache_hash_key(&crate::application::anonymous_blog_service::page)).await.unwrap();

    return ResultVO::success(true);
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

pub async  fn delete(blog_id: i64)->ResultVO<bool>{
    let _ = master().execute(Statement::from_sql_and_values(
        MySql,"update web_blog set deleted = 1 where blog_id = ? ",
        [sea_orm::Value::BigInt(Some(blog_id))])).await.unwrap();
    redis_master().await.del::<String,i64>(cache_hash_key(&page)).await.unwrap();
    redis_master().await.del::<String,i64>(cache_hash_key(&crate::application::anonymous_blog_service::page)).await.unwrap();

    return ResultVO::success(true);

}