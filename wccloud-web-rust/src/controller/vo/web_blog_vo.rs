//! author wcz
use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebBlogPageReqVO {
   pub page_num: u64,
   pub page_size: u64,
}


#[derive(Debug,Serialize,Deserialize,FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct WebBlogPageRespVO {
   pub blog_id: i64,
   pub title: String,
   pub img_url: String,
   pub summary: String,
   pub type_name: String,
   pub label_name: String,
   pub level: i8,
   pub enable_comment: i8,
   pub status: i8,
   pub update_time: DateTime,
}
#[derive(Debug,Serialize,Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebBlogSaveReqVO {
   pub blog_id: Option<String>,
   pub title: String,
   pub img_url: Option<String>,
   pub summary: String,
   pub type_name: String,
   pub label_name: Vec<String>,
   pub level: i8,
   pub enable_comment: bool,
   pub status: i8,
   pub html: String,
   pub md: String,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub struct WebBlogOneRespVO {
   pub blog_id: i64,
   pub title: String,
   pub img_url: String,
   pub summary: String,
   pub type_name: String,
   pub label_name: Option<Vec<String>>,
   pub level: i8,
   pub enable_comment: bool,
   pub status: i8,
   pub md: String,
   pub html: String,
}