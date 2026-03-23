use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::productos;

pub async fn get_productos(db: web::Data<DatabaseConnection>) -> impl Responder {
    match productos::Entity::find().all(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json("Obtener productos"),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener productos"),
    }
}