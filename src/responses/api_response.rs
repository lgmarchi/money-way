use std::collections::HashMap;

use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    message: String,
    extra: HashMap<String, Value>,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(
        message: &str,
        extra: HashMap<String, Value>,
        data: Option<T>,
    ) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: message.to_string(),
            extra,
            data,
        })
    }

    pub fn error(
        status_code: actix_web::http::StatusCode,
        message: &str,
    ) -> HttpResponse {
        HttpResponse::build(status_code).json(ApiResponse::<()> {
            status: "error".to_string(),
            message: message.to_string(),
            extra: HashMap::new(),
            data: None,
        })
    }
}
