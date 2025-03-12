use actix_web::{
    Responder,
    post,
    web,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SignUpRequest {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(data: web::Json<SignUpRequest>) -> impl Responder {
    format!("Sign Up: {:?}", data)
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign Up"
}
