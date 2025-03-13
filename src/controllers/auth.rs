use actix_web::{
    HttpResponse,
    Responder,
    post,
    web,
};
use serde::Deserialize;

use crate::{
    AppState,
    db,
};

#[derive(Deserialize, Debug)]
struct SignUpRequest {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(
    state: web::Data<AppState>,
    data: web::Json<SignUpRequest>,
) -> impl Responder {
    // Lock the mutex to get the database pool
    let db_pool = state.db.lock().await;

    if db::user::has_with_email(&db_pool, &data.email).await {
        return "Email already exists".to_string();
    }

    format!("Sign Up: {:?}", data)
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign Up"
}
