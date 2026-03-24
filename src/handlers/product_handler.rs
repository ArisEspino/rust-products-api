use actix_web::{HttpResponse, Responder, web};
use sea_orm::EntityTrait;

use crate::entities::productos;
use crate::models::product_models::{CrearProducto, Producto};
use crate::state::AppState;

pub async fn crear_producto(
    data: web::Data<AppState>,
    body: web::Json<CrearProducto>,
) -> impl Responder {
    let nombre = body.nombre.trim();

    if nombre.is_empty() {
        return HttpResponse::BadRequest().body("El nombre del producto no puede estar vacío");
    }

    HttpResponse::Ok().body("Producto creado exitosamente")
}

pub async fn listar_productos(data: web::Data<AppState>) -> impl Responder {
    let productos_db = productos::Entity::find().all(&data.db).await;
    match productos_db {
        Ok(productos) => HttpResponse::Ok().json(
            productos
                .into_iter()
                .map(|p| Producto {
                    id: p.id,
                    nombre: p.nombre,
                    precio: p.precio,
                    stock: p.stock,
                    creado_el: p.creado_el.to_string(),
                })
                .collect::<Vec<Producto>>(),
        ),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener los productos"),
    }
}
