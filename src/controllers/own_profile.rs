use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    get,
    post,
    web,
};

use crate::{
    AppState,
    db,
    domain::user::UpdateProfileRequest,
    utils::user_helpers::get_authenticated_user,
};

#[get("/own_profile")]
pub async fn get_own_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let db = state.db.lock().await;
    let user = get_authenticated_user(&db, &req).await;

    HttpResponse::Ok().json(user)
}

#[post("/own_profile")]
pub async fn update_profile(
    state: web::Data<AppState>,
    data: web::Json<UpdateProfileRequest>,
    req: HttpRequest,
) -> impl Responder {
    let db: tokio::sync::MutexGuard<'_, sqlx::Pool<sqlx::MySql>> =
        state.db.lock().await;

    let user = get_authenticated_user(&db, &req).await;
    let id_string = user.id.to_string();

    let _ =
        db::user_repository::update_by_id(&db, id_string.as_str(), &data).await;

    let user_updated =
        db::user_repository::get_by_id(&db, id_string.as_str()).await.unwrap();

    HttpResponse::Ok().json(user_updated)
}
