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
use serde_json::json;

use crate::{
    AppState,
    db::category_repository::CategoryRepository,
    domain::category::{
        CreateCategoryRequest,
        UpdateCategoryRequest,
    },
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
pub async fn create(
    state: web::Data<AppState>,
    data: web::Json<CreateCategoryRequest>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;
    let categories_repository = CategoryRepository::new(db.clone());
    let category = categories_repository.create(&data, user_id).await;

    HttpResponse::Ok().json(category)
}

#[get("/categories/{id}")]
pub async fn show(
    state: web::Data<AppState>,
    id: web::Path<u64>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;

    let categories_repository = CategoryRepository::new(db.clone());
    let category = categories_repository.get(id.into_inner()).await;

    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!(
        {
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    HttpResponse::Ok().json(category)
}

#[put("/categories/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    id: web::Path<u64>,
    data: web::Json<UpdateCategoryRequest>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;

    let categories_repository = CategoryRepository::new(db.clone());
    let category_id = *id;
    let category = categories_repository.get(id.into_inner()).await;

    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!(
        {
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    categories_repository.update(&data, category_id).await;
    let category_updated = categories_repository.get(category.id).await;

    HttpResponse::Ok().json(category_updated)
}

#[delete("/categories/{id}")]
pub async fn destroy(
    state: web::Data<AppState>,
    id: web::Path<u64>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;

    let categories_repository = CategoryRepository::new(db.clone());
    let category = categories_repository.get(id.into_inner()).await;

    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!(
        {
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    categories_repository.delete(category.id).await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Category deleted successfully.",
        "category_deleted": category
    }))
}
