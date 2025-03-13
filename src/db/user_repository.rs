use crate::{
    domain::user::{
        SignUpRequest,
        User,
    },
    utils::hash::hash_password,
};

pub async fn has_with_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    let result = sqlx::query!("SELECT email FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap();

    result.is_some()
}

pub async fn create(
    db: &sqlx::MySqlPool,
    sign_up_request: &SignUpRequest,
) -> bool {
    let hashed_password = hash_password(&sign_up_request.password);

    sqlx::query!(
        "INSERT INTO users (`email`, `password`, `first_name`, `last_name`) VALUES (?, ?, ?, ?)",
        sign_up_request.email,
        hashed_password,
        sign_up_request.first_name,
        sign_up_request.last_name
    ).execute(db).await.is_ok()
}

pub async fn get_by_email(db: &sqlx::MySqlPool, email: &str) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
        .fetch_one(db)
        .await
        .ok()
}
