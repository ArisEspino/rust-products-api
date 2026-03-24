mod entities;
mod handlers;
mod routes;
mod state;
mod models;
mod errors;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sea_orm::Database;
use state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurada");

    let db = Database::connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");

    let state = web::Data::new(AppState { db });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::product_routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
