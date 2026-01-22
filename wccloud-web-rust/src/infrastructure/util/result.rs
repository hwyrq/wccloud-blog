//! author wcz
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder};
use redis::{FromRedisValue, ParsingError, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResultVO<T: Serialize> {
    pub code: i8,
    pub msg: String,
    pub data: T,
}

#[allow(dead_code)]
pub trait SuccessMsgData<T: Serialize> {
    fn success(msg: String, data: T) -> ResultVO<T> {
        ResultVO { code: 0, msg, data }
    }
}

pub trait SuccessData<T: Serialize> {
    fn success(data: T) -> ResultVO<T> {
        ResultVO {
            code: 0,
            msg: "".to_string(),
            data,
        }
    }
}

impl<T: Serialize> SuccessMsgData<T> for ResultVO<T> {}

impl<T: Serialize> SuccessData<T> for ResultVO<T> {}

impl<T: Serialize> Responder for ResultVO<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string_pretty(&self).unwrap())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PageResultVO<T> {
    pub list: Vec<T>,
    pub total: u64,
}

impl<T: for<'a> Deserialize<'a>> FromRedisValue for PageResultVO<T> {
    fn from_redis_value(value: Value) -> Result<Self, ParsingError> {
        let bytes = match value {
            Value::BulkString(vec_bytes) => vec_bytes,
            Value::Nil => return Err("Received nil".into()),
            _ => return Err("Invalid value type".into()),
        };
        let s = String::from_utf8(bytes).map_err(|e| ParsingError::from(e))?;
        serde_json::from_str(&s).map_err(|e| ParsingError::from(e.to_string()))
    }
}

impl<T> PageResultVO<T> {
    pub fn new(list: Vec<T>, total: u64) -> PageResultVO<T> {
        PageResultVO { list, total }
    }
}
