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
    db::transaction_repository,
    domain::transaction::Transaction,
    utils,
};

#[get("/transactions")]
pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::user_helpers::get_user_id(&req);

    let transaction_repo =
        transaction_repository::TransactionRepository::new(db.clone());

    let transactions_vec: Vec<Transaction> =
        transaction_repo.get_all_of_user(user_id).await;

    HttpResponse::Ok().json(transactions_vec)
}

#[get("/transactions/{id}")]
pub async fn show() -> impl Responder {
    "Transactions: Show"
}

#[post("/transactions")]
pub async fn create() -> impl Responder {
    "Transactions: Create"
}

#[put("/transactions/{id}")]
pub async fn update() -> impl Responder {
    "Transactions: Update"
}

#[delete("/transactions/{id}")]
pub async fn destroy() -> impl Responder {
    "Transactions: Destroy"
}
