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
        self,
        category_repository::CategoryRepository,
        transaction_repository::{
            self,
            TransactionRepository,
        },
    },
    domain::transaction::{
        CreateTransactionRequest,
        Transaction,
        UpdateTransactionRequest,
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
        return HttpResponse::Forbidden().json(json!(
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
    let db = state.db.lock().await;
    let user = utils::user_helpers::get_authenticated_user(&db, &req).await;

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

    if category.user_id != user.id {
        return HttpResponse::Forbidden().json(json!(
        {
            "status": "error",
            "message": "You are not authorized to create transactions for this category"
        }));
    }

    if data.r#type.is_credit()
        && (user.balance < data.amount || category.balance < data.amount)
    {
        return HttpResponse::BadRequest().json(json!(
        {
            "status": "error",
            "message": "Insufficient balance"
        }
        ));
    }

    let transaction_repo = TransactionRepository::new(db.clone());
    let transaction_option = transaction_repo.create(&data, user.id).await;

    let (user_balance, category_balance) = if data.r#type.is_debit() {
        (user.balance - data.amount, category.balance - data.amount)
    } else {
        (user.balance + data.amount, category.balance + data.amount)
    };

    let _ =
        db::user_repository::update_balance(&db, user.id, user_balance).await;

    let _ = categories_repository
        .update_balance(category.id, category_balance)
        .await;

    match transaction_option {
        Some(transaction) => HttpResponse::Created().json(transaction),
        None => HttpResponse::BadRequest().json(json!(
        {
            "status": "error",
            "message": "Transaction not created"
        })),
    }
}

#[put("/transactions/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    id: web::Path<u64>,
    data: web::Json<UpdateTransactionRequest>,
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
        return HttpResponse::Forbidden().json(json!(
        {
            "status": "error",
            "message": "Unauthorized"
        }));
    };
    transaction_repo.update(&data, transaction.id).await;

    let transaction_option = transaction_repo.get(transaction.id).await;

    match transaction_option {
        Some(transaction) => HttpResponse::Ok().json(transaction),
        None => HttpResponse::BadRequest().json(json!(
        {
            "status": "error",
            "message": "Transaction not updated"
        })),
    }
}

#[delete("/transactions/{id}")]
pub async fn destroy(
    state: web::Data<AppState>,
    id: web::Path<u64>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = utils::user_helpers::get_user_id(&req);
    let db = state.db.lock().await;
    let user = utils::user_helpers::get_authenticated_user(&db, &req).await;

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
        return HttpResponse::Forbidden().json(json!(
        {
            "status": "error",
            "message": "Unauthorized"
        }));
    };

    let categories_repository = CategoryRepository::new(db.clone());
    let category =
        categories_repository.get(transaction.category_id).await.unwrap();

    if transaction.r#type.is_credit()
        && (transaction.amount > user.balance
            || transaction.amount > category.balance)
    {
        return HttpResponse::BadRequest().json(json!(
        {
            "status": "error",
            "message": "Insufficient balance"
        }
        ));
    }

    transaction_repo.delete(transaction.id).await;

    let (user_balance, category_balance) = if transaction.r#type.is_credit() {
        (
            user.balance - transaction.amount,
            category.balance - transaction.amount,
        )
    } else {
        (
            user.balance + transaction.amount,
            category.balance + transaction.amount,
        )
    };

    let _ =
        db::user_repository::update_balance(&db, user.id, user_balance).await;

    let _ = categories_repository
        .update_balance(category.id, category_balance)
        .await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Transaction deleted successfully.",
        "transaction_deleted": transaction
    }))
}
