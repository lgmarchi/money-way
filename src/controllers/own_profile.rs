use actix_web::{
    HttpMessage,
    HttpRequest,
    Responder,
    get,
    post,
};

use crate::utils::{
    self,
};

#[get("/own_profile")]
pub async fn get_own_profile(req: HttpRequest) -> impl Responder {
    let user_id = utils::req_user_id::get_user_id(&req);

    format!("Profile user id: {:#?}", user_id)
}

#[post("/own_profile")]
pub async fn update_profile() -> impl Responder {
    "Update Own Profile"
}
