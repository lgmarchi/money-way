pub async fn has_with_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    let result = sqlx::query!("SELECT email FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap();

    result.is_some()
}
