use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct ProductoRequest {
    pub nombre: String,
    pub precio: i32,
    pub stock: i32,
}

#[derive(Debug, Serialize)]
pub struct Producto {
    pub id: i32,
    pub nombre: String,
    pub precio: i32,
    pub stock: i32,
    pub creado_el: String,
}
