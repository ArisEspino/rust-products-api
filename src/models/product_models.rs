use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CrearProducto {
    pub nombre: String,
    pub precio: f64,
    pub stock: i32,
}

#[derive(Debug, Serialize)]
pub struct ListarProducto {
    pub id: i32,
    pub nombre: String,
    pub precio: f64,
    pub stock: i32,
    pub creado_el: String,
}
