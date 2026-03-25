mod config;
mod entities;
mod errors;
mod handlers;
mod models;
mod routes;
mod state;

use actix_web::{App, HttpServer, web};
use config::Config;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    let state = web::Data::new(AppState::new(&config.database_url, config.api_token).await);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::product_routes::config)
    })
    
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
    