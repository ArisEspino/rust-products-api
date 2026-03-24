use actix_web::web;
use crate::handlers:: product_handler;

pub fn config(cfg: &mut web::ServiceConfig) {

    cfg
    .route("/producto", web::post().to(product_handler::crear_producto))
    .route("/productos", web::get().to(product_handler::listar_productos))
    .route("/producto/{id}", web::get().to(product_handler::obtener_producto_por_id));
}   