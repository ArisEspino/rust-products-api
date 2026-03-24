use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Display;

#[derive(Display, Debug)]
pub enum ApiError {
    // #[display("Producto no encontrado.")]
    // NotFound,
    #[display("Datos de entrada inválidos.")]
    InvalidInput,
    #[display("No autorizado.")]
    Unauthorized,
    #[display("Error interno del servidor.")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
         
            ApiError::InvalidInput => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
