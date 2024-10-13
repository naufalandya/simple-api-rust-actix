// src/handlers/users.rs

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::user::{Info, UserResponse, CreateUser, GetUser};

pub async fn create_user(
    pool: web::Data<PgPool>,
    info: web::Json<CreateUser>,
) -> impl Responder {
    // Insert user ke database
    let result = sqlx::query!(
        "INSERT INTO users (name, age) VALUES ($1, $2) RETURNING id",
        info.name,
        info.age
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let response = UserResponse {
                message: format!("User {} berusia {} berhasil dibuat dengan ID {}.", info.name, info.age, record.id),
                status: 201,
            };
            HttpResponse::Created().json(response)
        },
        Err(e) => {
            eprintln!("Gagal membuat user: {}", e);
            HttpResponse::InternalServerError().body("Gagal membuat user")
        }
    }
}

pub async fn get_user(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query!(
        "SELECT id, name, age FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(record)) => {
            let response = serde_json::json!({
                "id": record.id,
                "name": record.name,
                "age": record.age,
            });
            HttpResponse::Ok().json(response)
        },
        Ok(None) => HttpResponse::NotFound().body("User tidak ditemukan"),
        Err(e) => {
            eprintln!("Gagal mengambil user: {}", e);
            HttpResponse::InternalServerError().body("Gagal mengambil user")
        }
    }
}
