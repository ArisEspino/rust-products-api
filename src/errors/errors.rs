use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Display, Debug)]
pub enum ApiError {
    #[display("Producto no encontrado.")]
    NotFound,
    #[display("Datos de entrada inválidos.")]
    InvalidInput,
    #[display("Error interno del servidor.")]
    InternalServerError,
}
