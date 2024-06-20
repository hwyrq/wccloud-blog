//! author wcz
use actix_web::{get, Responder, web};
use crate::application::web_type_service;

#[get("/type/list/name")]
pub async fn list_name()-> impl Responder {
   return web_type_service::list_name().await;
}
pub fn web_type_controller(cfg: &mut web::ServiceConfig){
    cfg.service(list_name);
}