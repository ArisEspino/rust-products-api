use actix_web::{HttpRequest, HttpResponse, Responder, web};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait};


use crate::entities::productos;
use crate::errors::errors::ApiError;
use crate::models::product_models::{ProductoRequest, Producto};
use crate::state::AppState;

pub async fn crear_producto(
    req: HttpRequest,
    data: web::Data<AppState>,
    body: web::Json<ProductoRequest>,
) -> Result<HttpResponse, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    let expected = format!("Bearer {}", data.api_token);

    match auth_header {
        Some(token) if token == expected => {}
        _ => return Err(ApiError::Unauthorized),
    }

    let nombre = body.nombre.trim();

    if nombre.is_empty() {
        return Err(ApiError::InvalidInput);
    }

    if body.precio <= 0 {
        return Err(ApiError::InvalidInput);
    }

    if body.stock <= 0 {
        return Err(ApiError::InvalidInput);
    }

    let nuevo_producto = productos::ActiveModel {
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

pub async fn obtener_producto_por_id(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let id = path
        .into_inner()
        .parse::<i32>()
        .map_err(|_| ApiError::InvalidInput)?;

    let producto_db = productos::Entity::find_by_id(id)
        .one(&data.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    match producto_db {
        Some(producto) => {
            let response = Producto {
                id: producto.id,
                nombre: producto.nombre,
                precio: producto.precio,
                stock: producto.stock,
                creado_el: producto.creado_el.to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        None => Err(ApiError::NotFound),
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

pub async fn actualizar_producto(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<ProductoRequest>,
) -> Result<HttpResponse, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    let expected = format!("Bearer {}", data.api_token);

    match auth_header {
        Some(token) if token == expected => {}
        _ => return Err(ApiError::Unauthorized),
    }

    let id = path
        .into_inner()
        .parse::<i32>()
        .map_err(|_| ApiError::InvalidInput)?;

    let nombre = body.nombre.trim();

    if nombre.is_empty() {
        return Err(ApiError::InvalidInput);
    }

    if body.precio <= 0 {
        return Err(ApiError::InvalidInput);
    }

    if body.stock < 0 {
        return Err(ApiError::InvalidInput);
    }

    let producto_encontrado = productos::Entity::find_by_id(id)
        .one(&data.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let producto_encontrado = match producto_encontrado {
        Some(producto) => producto,
        None => return Err(ApiError::NotFound),
    };

    let mut producto_activo: productos::ActiveModel = producto_encontrado.into();

    producto_activo.nombre = Set(nombre.to_string());
    producto_activo.precio = Set(body.precio);
    producto_activo.stock = Set(body.stock);

    let producto_actualizado = producto_activo
        .update(&data.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let response = Producto {
        id: producto_actualizado.id,
        nombre: producto_actualizado.nombre,
        precio: producto_actualizado.precio,
        stock: producto_actualizado.stock,
        creado_el: producto_actualizado.creado_el.to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn eliminar_producto(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    let expected = format!("Bearer {}", data.api_token);

    match auth_header {
        Some(token) if token == expected => {}
        _ => return Err(ApiError::Unauthorized),
    }

    let id = path
        .into_inner()
        .parse::<i32>()
        .map_err(|_| ApiError::InvalidInput)?;

    let producto = productos::Entity::find_by_id(id)
        .one(&data.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let producto = match producto {
        Some(producto) => producto,
        None => return Err(ApiError::NotFound),
    };

    producto
        .delete(&data.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok().body("Producto eliminado exitosamente"))
}
