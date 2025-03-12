use actix_web::{
    Responder,
    get,
    post,
};

#[get("/own_profile")]
pub async fn get_own_profile() -> impl Responder {
    "Get Own Profile"
}

#[post("/own_profile")]
pub async fn update_profile() -> impl Responder {
    "Update Own Profile"
}
