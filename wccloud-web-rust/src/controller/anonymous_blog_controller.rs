//! author wcz
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_multipart::Multipart;
use actix_web::{HttpRequest, Responder, get, post, web};
use bytes::Bytes;
use futures_util::{StreamExt, TryStreamExt};
use minio::s3::Client;
use minio::s3::builders::BucketExists;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::segmented_bytes::SegmentedBytes;
use minio::s3::types::{PartInfo, S3Api};
use redis::AsyncCommands;

use crate::application;
use crate::controller::vo::web_blog_vo::WebBlogPageReqVO;
use crate::infrastructure::config::get_config_value;
use crate::infrastructure::config::redis_config::redis_master;
use crate::infrastructure::util::result::{ResultVO, SuccessData};

#[get("/anonymous/blog/page")]
pub async fn page(item: web::Query<WebBlogPageReqVO>) -> impl Responder {
    application::anonymous_blog_service::page(&item.into_inner()).await
}

#[get("/anonymous/blog/one")]
pub async fn one(item: web::Query<HashMap<String, i64>>) -> impl Responder {
    application::web_blog_service::one(*item.get("blogId").unwrap()).await
}

#[get("/anonymous/blog/level")]
pub async fn level(item: web::Query<HashMap<String, i8>>) -> impl Responder {
    application::anonymous_blog_service::level(*item.get("level").unwrap()).await
}

#[get("/anonymous/blog/label")]
pub async fn label(_item: web::Query<HashMap<String, String>>) -> impl Responder {
    application::anonymous_blog_service::label().await
}

#[post("/file/upload")]
pub async fn upload(mut arg: Multipart, http_request: HttpRequest) -> impl Responder {
    // 获取 MinIO 配置
    let base_url_str = get_config_value::<String>("minio.url");
    let base_url: BaseUrl = base_url_str.parse().unwrap();
    let bucket_name: String = get_config_value("minio.bucket-name");
    let access_key = get_config_value::<String>("minio.access-key");
    let secret_key = get_config_value::<String>("minio.secret-key");

    // 创建 MinIO 客户端
    let static_provider = StaticProvider::new(&access_key, &secret_key, None);
    let client = Client::new(base_url, Some(Box::new(static_provider)), None, None).unwrap();

    // 检查 bucket 是否存在，不存在则创建
    let exists_response = BucketExists::new(client.clone(), bucket_name.clone())
        .send()
        .await
        .unwrap();

    if !exists_response.exists {
        // 创建 bucket
        let client_clone = client.clone();
        let bucket_name_clone = bucket_name.clone();
        client_clone
            .create_bucket(&bucket_name_clone)
            .send()
            .await
            .unwrap();
    }

    // 获取当前用户 ID (从 Redis)
    let token = http_request
        .headers()
        .get("token")
        .unwrap()
        .to_str()
        .unwrap();
    let user_id: String = redis_master()
        .await
        .get(&format!("accessToken:{}", token))
        .await
        .unwrap();

    let mut url = vec![];

    // 处理每个文件
    while let Ok(Some(mut field)) = arg.try_next().await {
        let content_disposition = field.content_disposition();
        let original_filename = content_disposition
            .unwrap()
            .get_filename()
            .unwrap_or_default();

        // 生成文件名格式: {user_id}/blog/{timestamp}_{filename}
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let filename = format!("{}/blog/{}_{}", user_id, timestamp, original_filename);

        // 读取文件数据
        let mut all_data: Vec<u8> = Vec::new();
        while let Some(Ok(b)) = field.next().await {
            all_data.extend_from_slice(&b);
        }

        // 开始分片上传
        let upload_id = client
            .create_multipart_upload(bucket_name.clone(), filename.clone())
            .send()
            .await
            .unwrap()
            .upload_id;

        // 上传分片 (作为单个分片上传)
        let part_number: u16 = 1;
        let upload_response = client
            .upload_part(
                bucket_name.clone(),
                filename.clone(),
                upload_id.clone(),
                part_number,
                SegmentedBytes::from(Bytes::from(all_data)),
            )
            .send()
            .await
            .unwrap();

        // 完成分片上传
        let parts = vec![PartInfo {
            number: part_number,
            etag: upload_response.etag,
            size: 0,
        }];

        let _complete_response = client
            .complete_multipart_upload(bucket_name.clone(), filename.clone(), upload_id, parts)
            .send()
            .await
            .unwrap();

        url.push(format!(
            "{}{}/{}",
            base_url_str.trim_end_matches('/'),
            bucket_name,
            filename
        ));
    }

    ResultVO::success(url)
}

#[get("/anonymous/actuator/health")]
pub async fn health(_item: web::Query<HashMap<String, String>>) -> impl Responder {
    "UP"
}

pub fn anonymous_blog_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(page)
        .service(one)
        .service(level)
        .service(label)
        .service(upload)
        .service(health);
}
