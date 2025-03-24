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
    db::{
        category_repository::CategoryRepository,
        transaction_repository::{
            self,
            TransactionRepository,
        },
    },
    domain::transaction::{
        CreateTransactionRequest,
        Transaction,
    },
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
pub async fn show(
    state: web::Data<AppState>,
    id: web::Path<u64>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;

    let transaction_repo = TransactionRepository::new(db.clone());
    let Some(transaction) = transaction_repo.get(id.into_inner()).await else {
        return HttpResponse::NotFound().json(json!(
        {
            "status": "error",
            "message": "Not found"
        }
        ));
    };

    if transaction.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!(
        {
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    HttpResponse::Ok().json(transaction)
}

#[post("/transactions")]
pub async fn create(
    state: web::Data<AppState>,
    data: web::Json<CreateTransactionRequest>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;

    let categories_repository = CategoryRepository::new(db.clone());
    let Some(category) = categories_repository.get(data.category_id).await
    else {
        return HttpResponse::NotFound().json(json!(
        {
            "status": "error",
            "message": "Category Not found"
        }
        ));
    };

    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!(
        {
            "status": "error",
            "message": "You are not authorized to create transactions for this category"
        }));
    }

    let transaction_repo = TransactionRepository::new(db.clone());
    let transaction_option = transaction_repo.create(&data, user_id).await;

    match transaction_option {
        Some(transaction) => HttpResponse::Ok().json(transaction),
        None => HttpResponse::Unauthorized().json(json!(
        {
            "status": "error",
            "message": "Transaction not created"
        })),
    }
}

#[put("/transactions/{id}")]
pub async fn update() -> impl Responder {
    "Transactions: Update"
}

#[delete("/transactions/{id}")]
pub async fn destroy() -> impl Responder {
    "Transactions: Destroy"
}
