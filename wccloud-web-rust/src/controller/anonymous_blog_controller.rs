//! author wcz
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_multipart::Multipart;

use actix_web::{get, post, Responder, web};

use crate::application;
use crate::controller::vo::web_blog_vo::WebBlogPageReqVO;
use crate::infrastructure::util::result::{ResultVO, SuccessData};

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

use futures_util::{StreamExt, TryStreamExt};
use minio::s3::args::{BucketExistsArgs, CompleteMultipartUploadArgs, ComposeObjectArgs, CreateMultipartUploadArgs, MakeBucketArgs, UploadObjectArgs, UploadPartArgs, UploadPartCopyArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::Part;
use minio::s3::utils::Multimap;
use crate::infrastructure::util::constant::{MINIO_ACCESS_KEY, MINIO_SECRET_KEY};

#[post("/file/upload")]
pub async fn upload(mut arg: Multipart) -> impl Responder {
    let base_url = "http://home0122:9000".parse::<BaseUrl>().unwrap();
    let static_provider = StaticProvider::new(&**MINIO_ACCESS_KEY, &**MINIO_SECRET_KEY, None);
    let client = Client::new(base_url.clone(), Some(Box::new(static_provider)), None, Some(true)).unwrap();
    let bucket_name = "wccloud";
    let exists = client.bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap()).await.unwrap();
    if !exists { client.make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap()).await.unwrap(); }
    let mut url = vec![];
    while let Ok(Some(mut field)) = arg.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string() + "_" +
            content_disposition.get_filename().unwrap();

        let response = client.create_multipart_upload_old(&CreateMultipartUploadArgs::new(&bucket_name, &*filename).unwrap()).await.unwrap();
        let mut vec1: Vec<u8> = vec![];
        let mut num = 1;
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