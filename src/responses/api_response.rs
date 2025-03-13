use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: message.to_string(),
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
            data: None,
        })
    }
}
