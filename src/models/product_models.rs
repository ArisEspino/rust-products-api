use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
pub struct CrearProducto {
    pub nombre: String,
    pub precio: Decimal,
    pub stock: i32,
}

// #[derive(Debug, Deserialize)]
// pub struct ActualizarProducto {
//     pub nombre: String,
//     pub precio: Decimal,
//     pub stock: i32,
// }

#[derive(Debug, Serialize)]
pub struct Producto {
    pub id: i32,
    pub nombre: String,
    pub precio: Decimal,
    pub stock: i32,
    pub creado_el: String,
}
