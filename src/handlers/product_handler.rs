use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use rust_decimal::Decimal;

use crate::entities::productos;
use crate::errors::errors::ApiError;
use crate::models::product_models::{CrearProducto, Producto};
use crate::state::AppState;

pub async fn crear_producto(
    req: HttpRequest,
    data: web::Data<AppState>,
    body: web::Json<CrearProducto>,
) -> Result<HttpResponse, ApiError> {
    let auth_header = req.headers().get("Authorization").and_then(|value| value.to_str().ok());

    let expected = format!("Bearer {}", data.api_token);    

    match auth_header {
        Some(token) if token == expected => {}
        _ => return Err(ApiError::Unauthorized),
    } 

    let nombre = body.nombre.trim();

    if nombre.is_empty(){
        return Err(ApiError::InvalidInput);
    }

    if body.precio <= Decimal::new(0,0){
        return Err(ApiError::InvalidInput);
    }

    if body.stock < 0 {
        return Err(ApiError::InvalidInput);
    }

    let nuevo_producto = productos::ActiveModel{
        nombre: Set(nombre.to_string()),
        precio: Set(body.precio),
        stock: Set(body.stock),
        ..Default::default()
    };

    match nuevo_producto.insert(&data.db).await {
        Ok(producto_guardado) => {
            let response = Producto {
                id: producto_guardado.id,
                nombre: producto_guardado.nombre,
                precio: producto_guardado.precio,
                stock: producto_guardado.stock,
                creado_el: producto_guardado.creado_el.to_string(),
            };
            Ok(HttpResponse::Created().json(response))
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
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
