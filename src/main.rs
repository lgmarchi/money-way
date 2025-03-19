use actix_web::{
    App,
    HttpServer,
    middleware::{
        Logger,
        from_fn,
    },
    web::{
        self,
    },
};
use env_logger::Env;

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

    env_logger::init_from_env(Env::default().default_filter_or("info"));

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
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
            .service(
                web::scope("/api")
                    .wrap(from_fn(middleware::auth::verify_jwt))
                    .service(controllers::own_profile::get_own_profile)
                    .service(controllers::own_profile::update_profile)
                    .service(controllers::categories::index)
                    .service(controllers::categories::show)
                    .service(controllers::categories::update)
                    .service(controllers::categories::create)
                    .service(controllers::categories::destroy),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
