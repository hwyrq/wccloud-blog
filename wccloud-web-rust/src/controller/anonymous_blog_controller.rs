//! author wcz
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_multipart::Multipart;
use actix_web::{get, HttpRequest, post, Responder, web};
use futures_util::{StreamExt, TryStreamExt};
use minio::s3::args::{BucketExistsArgs, CompleteMultipartUploadArgs, CreateMultipartUploadArgs, MakeBucketArgs, UploadPartArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::Part;
use redis::AsyncCommands;

use crate::application;
use crate::controller::vo::web_blog_vo::WebBlogPageReqVO;
use crate::infrastructure::config::get_config_value;
use crate::infrastructure::config::redis_config::redis_master;
use crate::infrastructure::util::result::{ResultVO, SuccessData};

#[get("/anonymous/blog/page")]
pub async fn page(item: web::Query<WebBlogPageReqVO>) -> impl Responder {
    return application::anonymous_blog_service::page(&item.into_inner()).await;
}

#[get("/anonymous/blog/one")]
pub async fn one(item: web::Query<HashMap<String, i64>>) -> impl Responder {
    return application::web_blog_service::one(*item.get("blogId").unwrap()).await;
}

#[get("/anonymous/blog/level")]
pub async fn level(item: web::Query<HashMap<String, i8>>) -> impl Responder {
    return application::anonymous_blog_service::level(*item.get("level").unwrap()).await;
}

#[get("/anonymous/blog/label")]
pub async fn label(_item: web::Query<HashMap<String, String>>) -> impl Responder {
    return application::anonymous_blog_service::label().await;
}

#[post("/file/upload")]
pub async fn upload(mut arg: Multipart,http_request: HttpRequest) -> impl Responder {
    let base_url = get_config_value::<String>("minio.url").parse::<BaseUrl>().unwrap();
    let bucket_name:String = get_config_value("minio.bucket-name");
    let access_key = get_config_value::<String>("minio.access-key");
    let secret_key = get_config_value::<String>("minio.secret-key");
    let static_provider = StaticProvider::new(&*access_key, &*secret_key, None);
    let client = Client::new(base_url.clone(), Some(Box::new(static_provider)), None, Some(true)).unwrap();
    let exists = client.bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap()).await.unwrap();
    if !exists { client.make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap()).await.unwrap(); }
    //获取当前用户id
    let user_id = redis_master().await.get::<&str,String>(
        &*("accessToken:".to_string() + http_request.headers().get("token").unwrap().to_str().unwrap())
    ).await.unwrap();
    let mut url = vec![];
    while let Ok(Some(mut field)) = arg.try_next().await {
        let content_disposition = field.content_disposition();
        //文件名格式 {user_id}/blog/{timestamp}_{filename}
        let filename = user_id.clone() + 
            "/blog/" + 
            &*SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string() + 
            "_" +
            content_disposition.get_filename().unwrap();

        let response = client.create_multipart_upload_old(&CreateMultipartUploadArgs::new(&bucket_name, &*filename).unwrap()).await.unwrap();
        let mut vec1: Vec<u8> = vec![];
        let  num = 1;
        let mut parts: Vec<Part> = Vec::new();
        while let Some(Ok(b)) = field.next().await {
            let mut vec2 = b.to_vec();
            vec1.append(&mut vec2)
        }
        let x = vec1.iter().as_slice();
        let object_base_response = client.upload_part_old(&UploadPartArgs::new(&bucket_name, &*filename, &*response.upload_id, num, x).unwrap()).await.unwrap();
        parts.push(Part { number: num, etag: object_base_response.etag });
        let put_object_base_response = client.complete_multipart_upload_old(&CompleteMultipartUploadArgs::new(&bucket_name, &*filename, &*response.upload_id, &parts).unwrap()).await.unwrap();
        url.push(put_object_base_response.location);
    }
    return ResultVO::success(url);
}

pub fn anonymous_blog_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(one)
        .service(level)
        .service(label)
        .service(upload);
}