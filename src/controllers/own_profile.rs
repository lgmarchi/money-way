use actix_web::{
    HttpMessage,
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
    utils::{
        self,
    },
};

#[get("/own_profile")]
pub async fn get_own_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::req_user_id::get_user_id(&req);
    let user_id_string = user_id.to_string();
    let id = user_id_string.as_str();
    let user = db::user_repository::get_by_id(&db, id).await;

    HttpResponse::Ok().json(user)
}

#[post("/own_profile")]
pub async fn update_profile() -> impl Responder {
    "Update Own Profile"
}
