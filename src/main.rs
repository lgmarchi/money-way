use actix_web::{
    App,
    HttpServer,
    web,
};

mod controllers;

struct AppState {
    db: tokio::sync::Mutex<sqlx::MySqlPool>,
}

const DATABASE_URL: &str = "DATABASE_URL";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let state: web::Data<AppState> = web::Data::new(AppState {
        db: tokio::sync::Mutex::new(
            sqlx::MySqlPool::connect(DATABASE_URL).await.unwrap(),
        ),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
