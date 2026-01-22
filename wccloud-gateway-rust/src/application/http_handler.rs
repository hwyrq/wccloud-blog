use actix_web::{web, HttpRequest, HttpResponse, Result};

use crate::application::reverse_proxy::ReverseProxy;

pub async fn handle_proxy_request(
    req: HttpRequest,
    body: web::Bytes,
    app_state: web::Data<crate::AppState>,
) -> Result<HttpResponse> {
    let path = req.uri().path();

    // 根据路径查找路由
    if let Some(route) = app_state.route_table.get_route_by_path(path) {
        // 应用过滤器（StripPrefix）
        let stripped_path = apply_filters(path, &route.filters);

        // 使用反向代理发送请求
        let proxy = ReverseProxy {
            naming_service: app_state.naming_service.as_ref().clone(),
        };

        proxy
            .forward_request_with_stripped_path(
                &app_state.http_client,
                route.uri.clone(), // original uri with lb:// format
                stripped_path,
                req.to_owned(),
                body,
            )
            .await
    } else {
        // 如果没有找到匹配的路由，返回404
        Ok(HttpResponse::NotFound().json("Route not found"))
    }
}

// 应用过滤器，目前仅处理StripPrefix
pub fn apply_filters(path: &str, filters: &[crate::domain::models::FilterConfig]) -> String {
    for filter in filters {
        if let Some(strip_prefix) = filter.strip_prefix {
            let path_parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

            if path_parts.len() > strip_prefix as usize {
                let new_path = path_parts[strip_prefix as usize..].join("/");
                return format!("/{}", new_path);
            } else {
                return "/".to_string();
            }
        }
    }

    path.to_string()
}
