use crate::domain::user::SignUpRequest;

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
    let hashed_password =
        bcrypt::hash(sign_up_request.password.clone(), bcrypt::DEFAULT_COST)
            .unwrap();

    sqlx::query!(
        "INSERT INTO users (`email`, `password`, `first_name`, `last_name`) VALUES (?, ?, ?, ?)",
        sign_up_request.email,
        hashed_password,
        sign_up_request.first_name,
        sign_up_request.last_name
    ).execute(db).await.is_ok()
}
