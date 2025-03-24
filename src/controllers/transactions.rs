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

#[get("/transactions")]
pub async fn index() -> impl Responder {
    "Transactions: List"
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
