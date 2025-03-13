use actix_web::{
    HttpMessage,
    HttpRequest,
    Responder,
    get,
    post,
};

#[get("/own_profile")]
pub async fn get_own_profile(req: HttpRequest) -> impl Responder {
    let extension: std::cell::Ref<'_, actix_web::dev::Extensions> =
        req.extensions();

    let user_id = extension.get::<u64>().unwrap();

    format!("Profile user id: {:#?}", user_id)
}

#[post("/own_profile")]
pub async fn update_profile() -> impl Responder {
    "Update Own Profile"
}
