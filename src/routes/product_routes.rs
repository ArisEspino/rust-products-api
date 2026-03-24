use actix_web::web;
use crate::handlers:: product_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/productos", web::get().to(product_handler::listar_productos));
}