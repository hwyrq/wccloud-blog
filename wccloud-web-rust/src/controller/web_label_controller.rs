//! author wcz
use actix_web::{get, Responder, web};
use crate::application::web_label_service;

///@author wcz
#[get("/label/list/name")]
pub async fn list_name()-> impl Responder {
    return web_label_service::list_name().await;
}

pub fn web_label_controller(cfg: &mut web::ServiceConfig){
    cfg.service(list_name);
}

