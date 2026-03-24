mod entities;
mod errors;
mod handlers;
mod models;
mod routes;
mod state;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sea_orm::Database;
use state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurada");

    let api_token = env::var("API_TOKEN").expect("API_TOKEN no está configurada");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let db = Database::connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");

    let state = web::Data::new(AppState { db, api_token });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::product_routes::config)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
