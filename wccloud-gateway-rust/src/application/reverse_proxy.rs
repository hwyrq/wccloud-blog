use actix_web::http::header::{HeaderName as ActixHeaderName, HeaderValue as ActixHeaderValue};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use reqwest::header::{
    HeaderMap as ReqwestHeaderMap, HeaderName as ReqwestHeaderName,
    HeaderValue as ReqwestHeaderValue,
};
use reqwest::{Client, Method as ReqwestMethod};
use url;

pub struct ReverseProxy {
    pub naming_service: nacos_sdk::api::naming::NamingService,
}

impl ReverseProxy {
    pub async fn forward_request_with_stripped_path(
        &self,
        client: &Client,
        original_target_url: String, // Original URL with lb:// if used for service discovery
        stripped_path: String,       // Path after applying filters like StripPrefix
        req: HttpRequest,
        body: web::Bytes,
    ) -> Result<HttpResponse> {
        let actual_url = if original_target_url.starts_with("lb://") {
            // Extract service name from "lb://wccloud-admin-server"
            let service_name = original_target_url.strip_prefix("lb://").unwrap();

            match self.get_service_instance(service_name).await {
                Some(instance) => {
                    // Validate that IP and port are valid
                    if instance.ip.trim().is_empty() {
                        tracing::error!(
                            "Invalid IP in service instance for {}: {:?}",
                            service_name,
                            instance
                        );
                        return Err(actix_web::error::ErrorBadGateway(format!(
                            "Invalid service instance IP for: {}",
                            service_name
                        )));
                    }

                    if instance.port <= 0 || instance.port > 65535 {
                        tracing::error!(
                            "Invalid port in service instance for {}: {:?}",
                            service_name,
                            instance
                        );
                        return Err(actix_web::error::ErrorBadGateway(format!(
                            "Invalid service instance port for: {}",
                            service_name
                        )));
                    }

                    let service_url = format!("http://{}:{}", instance.ip, instance.port);

                    tracing::info!(
                        "Found service instance: {} -> {} (port: {}, health: {:?})",
                        service_name,
                        service_url,
                        instance.port,
                        instance.healthy
                    );

                    // Verify the URL is formatted properly before using it
                    if let Err(_) = url::Url::parse(&service_url) {
                        tracing::error!("Invalid service URL format: {}", service_url);
                        return Err(actix_web::error::ErrorBadGateway(format!(
                            "Invalid service URL: {}",
                            service_url
                        )));
                    }
                    service_url
                }
                None => {
                    tracing::error!("Service not found in Nacos: {}", service_name);
                    return Err(actix_web::error::ErrorBadGateway(format!(
                        "Service not found: {}",
                        service_name
                    )));
                }
            }
        } else {
            // If not using service discovery, use direct URL
            tracing::info!("Using direct URL: {}", original_target_url);
            original_target_url
        };

        // We use the already-stripped path instead of extracting it from the request URI

        // This handles filters like StripPrefix properly

        // Extract query string and reconstruct the full path with query parameters
        let original_query_string = req
            .uri()
            .query()
            .map(|q| format!("?{}", q))
            .unwrap_or_default();
        let actual_target_url = format!("{}{}{}", actual_url, stripped_path, original_query_string);
        tracing::info!("Original URI: {}", req.uri());
        tracing::info!("Constructed target URL: {}", actual_target_url);

        // Validate that the URL is well-formed
        if !actual_target_url.starts_with("http://") && !actual_target_url.starts_with("https://") {
            tracing::error!("Invalid target URL format: {}", actual_target_url);
            return Err(actix_web::error::ErrorBadGateway(format!(
                "Invalid target URL: {}",
                actual_target_url
            )));
        }

        tracing::info!("Forwarding request to: {}", actual_target_url);

        // Convert Actix HTTP method to Reqwest method
        let method = convert_method(req.method());

        // Manually copy headers from the incoming request to reqwest's request
        let mut headers = ReqwestHeaderMap::new();
        for (key, value) in req.headers().iter() {
            if let (Ok(header_name), Ok(header_value)) = (
                ReqwestHeaderName::try_from(key.as_str()),
                ReqwestHeaderValue::try_from(value.as_bytes()),
            ) {
                headers.insert(header_name, header_value);
            }
        }

        // Log request information before sending
        tracing::info!("Request method: {:?}", method);
        tracing::debug!("Request headers: {:?}", headers);
        tracing::info!("Request body length: {}", body.len());
        tracing::info!("Target URL: {}", actual_target_url);

        // Create request builder
        let request_builder = client
            .request(method.clone(), &actual_target_url)
            .headers(headers.clone())
            .body(body.to_vec());

        // Create and send the request

        let response = request_builder.send().await.map_err(|e| {
            tracing::error!("Failed to send request to {}: {}", actual_target_url, e);

            tracing::info!(
                "Request method: {:?}, Body length: {}, Headers: {:?}",
                method,
                body.len(),
                headers
            );
            actix_web::error::ErrorBadGateway(format!("Failed to forward request: {}", e))
        })?;

        // Convert the response to HttpResponse
        let status =
            actix_web::http::StatusCode::from_u16(response.status().as_u16()).map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Invalid status code: {}", e))
            })?;

        let mut response_builder = HttpResponse::build(status);

        // Copy headers from the upstream response to the response we're returning
        for (name, value) in response.headers() {
            if let Some(header_name) = ActixHeaderName::try_from(name.as_str()).ok() {
                if let Some(header_value) = ActixHeaderValue::try_from(value.as_bytes()).ok() {
                    response_builder.insert_header((header_name, header_value));
                }
            }
        }

        // Get the response body
        let response_body = response.bytes().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to read response body: {}",
                e
            ))
        })?;

        Ok(response_builder.body(response_body))
    }

    // Helper method to query service instance from Nacos
    async fn get_service_instance(
        &self,
        service_name: &str,
    ) -> Option<nacos_sdk::api::naming::ServiceInstance> {
        match self
            .naming_service
            .select_one_healthy_instance(
                service_name.to_string(),
                Some(nacos_sdk::api::constants::DEFAULT_GROUP.to_string()),
                Vec::default(),
                true,
            )
            .await
        {
            Ok(instance) => Some(instance),
            Err(e) => {
                tracing::error!(
                    "Failed to get service instance from Nacos for service {}: {}",
                    service_name,
                    e
                );
                None
            }
        }
    }
}

fn convert_method(method: &actix_web::http::Method) -> ReqwestMethod {
    match *method {
        actix_web::http::Method::GET => ReqwestMethod::GET,
        actix_web::http::Method::POST => ReqwestMethod::POST,
        actix_web::http::Method::PUT => ReqwestMethod::PUT,
        actix_web::http::Method::DELETE => ReqwestMethod::DELETE,
        actix_web::http::Method::HEAD => ReqwestMethod::HEAD,
        actix_web::http::Method::OPTIONS => ReqwestMethod::OPTIONS,
        actix_web::http::Method::PATCH => ReqwestMethod::PATCH,
        _ => ReqwestMethod::GET, // Default to GET for unsupported methods
    }
}
