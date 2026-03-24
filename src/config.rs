use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub api_token: String,
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL no está configurada"),
            api_token: env::var("API_TOKEN").expect("API_TOKEN no está configurada"),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
        }
    }
}
