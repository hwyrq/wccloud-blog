//! author wcz
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct WebLabelResp {
    pub label_id: i64,
    pub label_name: String,
}