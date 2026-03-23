use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sea_orm::{Database};
use std::env;

mod entities;
mod handlers {
    pub mod product_handler;
}
use handlers::product_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurada");

    let db = Database::connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/productos", web::get().to(product_handler::get_productos))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
