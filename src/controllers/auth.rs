use std::collections::HashMap;

use actix_web::{
    Responder,
    post,
    web,
};
use jsonwebtoken::EncodingKey;
use serde_json::json;

use crate::{
    AppState,
    db,
    domain::user::{
        self,
        SignInRequest,
        SignUpRequest,
    },
    responses::api_response::ApiResponse,
    utils::hash::verify_password,
};

#[post("/auth/sign-up")]
pub async fn sign_up(
    state: web::Data<AppState>,
    data: web::Json<SignUpRequest>,
) -> impl Responder {
    // Lock the mutex to get the database pool
    let db_pool = state.db.lock().await;

    if db::user_repository::has_with_email(&db_pool, &data.email).await {
        return ApiResponse::<()>::error(
            actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
            "Email already in use",
        );
    }

    db::user_repository::create(&db_pool, &data).await;

    ApiResponse::<web::Json<SignUpRequest>>::success(
        "Account created successfully",
        HashMap::new(),
        Some(data),
    )
}

#[post("/auth/sign-in")]
pub async fn sign_in(
    state: web::Data<AppState>,
    data: web::Json<SignInRequest>,
) -> impl Responder {
    let db_pool = state.db.lock().await;

    let Some(user) =
        db::user_repository::get_by_email(&db_pool, &data.email).await
    else {
        return ApiResponse::<()>::error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Invalid email or password",
        );
    };

    if !verify_password(&data.password, &user.password) {
        return ApiResponse::<()>::error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Invalid email or password",
        );
    }

    let claims = user::Claims::new(user.id, "user".to_string(), None);

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .unwrap();

    let mut extra_fields = HashMap::new();
    extra_fields.insert("token".to_string(), json!(token));

    ApiResponse::<String>::success(
        "Token generated successfully",
        extra_fields,
        None,
    )
}
