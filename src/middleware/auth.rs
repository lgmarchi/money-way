use actix_web::{
    Error,
    HttpMessage,
    body::BoxBody,
    dev::{
        ServiceRequest,
        ServiceResponse,
    },
    error::ErrorUnauthorized,
    middleware::Next,
    web,
};
use jsonwebtoken::{
    DecodingKey,
    Validation,
    decode,
};
use serde_json::json;

use crate::{
    AppState,
    controllers::auth,
    domain::user::Claims,
    responses::api_response::ApiResponse,
};

pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header is missing",

        }))
    })?;

    let auth_str = auth_header.to_str().map_err(|_| {
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header is malformed",

        }))
    })?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header is malformed",

        })));
    }

    let token = auth_str.strip_prefix("Bearer ").unwrap();

    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let key = DecodingKey::from_secret(&state.jwt_secret.as_bytes());

    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims.sub);

            next.call(req).await
        }
        Err(_) => {
            return Err(ErrorUnauthorized(json!({
                "status": "error",
                "message": "Invalid Token",

            })));
        }
    }
}
