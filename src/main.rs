use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
// use model::AppState;
use dotenv::dotenv;
use std::env;

mod handler;
mod handler2;
mod model;
mod response;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "actix_web=info");
    // }
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // let todo_db = AppState::init();
    // let app_data = web::Data::new(todo_db);

    dotenv().ok();
    println!("Server started successfully");

    let db_url = env::var("DATABASE_URL").expect("Expected a token in the environment");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Not found");
    let app_data = web::Data::new(pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .configure(handler2::config)
            .wrap(cors)
            .wrap(handler2::SayHi)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
