use actix_web::{
    App,
    HttpServer,
    web,
};

mod controllers;
mod db;
mod domain;
mod middleware;
mod responses;
mod utils;

struct AppState {
    db: tokio::sync::Mutex<sqlx::MySqlPool>,
    jwt_secret: String,
}

const DATABASE_URL: &str = "DATABASE_URL";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = &std::env::var(DATABASE_URL).unwrap();

    let db_connection = match sqlx::MySqlPool::connect(database_url).await {
        Ok(pool) => pool,
        Err(error) => {
            println!("Error to: {:?}", error);
            panic!("Not possible to proceed without db pool");
        }
    };

    let state: web::Data<AppState> = web::Data::new(AppState {
        db: tokio::sync::Mutex::new(db_connection),
        jwt_secret: std::env::var("JWT_SECRET").unwrap(),
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
