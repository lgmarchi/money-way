use actix_web::{
    HttpMessage,
    HttpRequest,
};

pub fn get_user_id(req: &HttpRequest) -> u64 {
    let extension: std::cell::Ref<'_, actix_web::dev::Extensions> =
        req.extensions();

    *extension.get::<u64>().unwrap()
}
