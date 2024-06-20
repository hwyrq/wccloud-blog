use sea_orm::ConnectionTrait;

use crate::controller::vo::web_blog_vo::WebBlogSaveReqVO;
use crate::infrastructure::dao::web_type_dao;

pub(crate) async fn process_type(arg: &WebBlogSaveReqVO, type_id:&mut i64, user_id: i64, x: &impl ConnectionTrait) {
//判断当前type是否存在
    let value = sea_orm::Value::from(arg.type_name.to_string());
    let option_web_tpe = web_type_dao::get_web_type(&value,x).await;
    match option_web_tpe {
        None => {
            web_type_dao::save_web_type(arg, *type_id, user_id, x).await;
        }
        Some(model) => {
            *type_id = model.type_id;
            //并吧deleted设置为0
            web_type_dao::recover_web_type(value,x).await;
        }
    }
}