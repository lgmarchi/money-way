use actix_web::{
    HttpMessage,
    HttpRequest,
};

use crate::{
    db,
    domain::user::User,
};

pub fn get_user_id(req: &HttpRequest) -> u64 {
    let extension: std::cell::Ref<'_, actix_web::dev::Extensions> =
        req.extensions();

    *extension.get::<u64>().unwrap()
}

pub async fn get_authenticated_user(
    db: &sqlx::MySqlPool,
    req: &HttpRequest,
) -> User {
    db::user_repository::get_by_id(db, get_user_id(req).to_string().as_str())
        .await
        .unwrap()
}
