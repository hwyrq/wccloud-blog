use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RouteConfig {
    pub id: String,

    pub uri: String,

    pub predicates: Vec<PredicateConfig>,

    pub filters: Vec<FilterConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct PredicateConfig {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct FilterConfig {
    #[serde(rename = "strip_prefix")]
    pub strip_prefix: Option<u32>,
}
