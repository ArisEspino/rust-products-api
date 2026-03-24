use sea_orm::{Database, DatabaseConnection};

pub struct AppState {
    pub db: DatabaseConnection,
    pub api_token: String,
}

impl AppState {
    pub async fn new(database_url: &str, api_token: String) -> Self {
        let db = Database::connect(database_url)
            .await
            .expect("Error al conectar a la base de datos");

        Self { db, api_token }
    }
}
