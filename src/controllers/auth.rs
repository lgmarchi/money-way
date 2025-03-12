use actix_web::{
    HttpResponse,
    Responder,
    post,
    web,
};
use serde::Deserialize;

use crate::AppState;

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

    // Get a connection from the pool
    match sqlx::query!(
        "INSERT INTO users (email, password) VALUES (?, ?)",
        data.email,
        data.password
    )
    .execute(&*db_pool) // Deref to access Pool<MySql>
    .await
    {
        Ok(_) => {
            HttpResponse::Ok().body(format!("User {} signed up!", data.email));
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e));
        }
    }

    format!("Sign Up: {:?}", data)
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign Up"
}
