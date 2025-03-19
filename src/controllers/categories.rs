use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    delete,
    get,
    post,
    put,
    web,
};

use crate::{
    AppState,
    db::category_repository::CategoryRepository,
    utils,
};

#[get("/categories")]
pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;
    let categories_repository = CategoryRepository::new(db.clone());
    let categories = categories_repository.get_all_of_user(user_id).await;

    HttpResponse::Ok().json(categories)
}
#[post("/categories")]
pub async fn create() -> impl Responder {
    "Categories: Create"
}

#[get("/categories/{id}")]
pub async fn show() -> impl Responder {
    "Categories: Show"
}

#[put("/categories/{id}")]
pub async fn update() -> impl Responder {
    "Categories: Update"
}

#[delete("/categories/{id}")]
pub async fn destroy() -> impl Responder {
    "Categories: Destroy"
}
