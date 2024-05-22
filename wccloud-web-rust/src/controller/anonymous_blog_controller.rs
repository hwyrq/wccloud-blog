//! author wcz
use std::collections::HashMap;

use actix_web::{get, Responder, web};

use crate::application;
use crate::controller::vo::web_blog_vo::WebBlogPageReqVO;

#[get("/anonymous/blog/page")]
pub async fn page(item: web::Query<WebBlogPageReqVO>) -> impl Responder {
    return application::anonymous_blog_service::page(&item.into_inner()).await;
}
#[get("/anonymous/blog/one")]
pub async fn one(item: web::Query<HashMap<String, i64>>) -> impl Responder {
    return application::anonymous_blog_service::one(*item.get("blogId").unwrap()).await;
}
#[get("/anonymous/blog/level")]
pub async fn level(item: web::Query<HashMap<String, i8>>) -> impl Responder {
    return application::anonymous_blog_service::level(*item.get("level").unwrap()).await;
}
#[get("/anonymous/blog/label")]
pub async fn label(_item: web::Query<HashMap<String, String>>) -> impl Responder {
    return application::anonymous_blog_service::label().await;
}
pub fn anonymous_blog_controller(cfg: &mut web::ServiceConfig){
    cfg.service(page)
        .service(one)
        .service(level)
        .service(label);
}