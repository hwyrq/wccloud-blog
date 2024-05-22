//! author wcz
use std::collections::HashMap;

use actix_web::{delete, get, post, Responder, web};

use crate::application;
use crate::controller::vo::web_blog_vo::{WebBlogPageReqVO, WebBlogSaveReqVO};

/// author wcz
#[get("/blog/page")]
pub async fn page(item: web::Query<WebBlogPageReqVO>) -> impl Responder {
    return application::web_blog_service::page(&item.into_inner()).await;
}

#[post("/blog/save")]
pub async fn save(item: web::Json<WebBlogSaveReqVO>) -> impl Responder {
    return application::web_blog_service::save(&item.into_inner()).await;
}

#[get("/blog/one")]
pub async fn one(item: web::Query<HashMap<String, i64>>) -> impl Responder {
    return application::web_blog_service::one(*item.get("blogId").unwrap()).await;
}

#[delete("/blog/delete")]
pub async  fn delete(item: web::Query<HashMap<  String, i64> >)->impl Responder{
    return application::web_blog_service::delete(*item.get("blogId").unwrap()).await;
}

///web_blog_controller
pub fn web_blog_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(save)
        .service(one)
        .service(delete);
}