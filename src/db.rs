// src/db.rs

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

/// Membuat dan menginisialisasi pool koneksi PostgreSQL
pub async fn establish_connection() -> PgPool {
    dotenv().ok(); // Memuat variabel lingkungan dari .env

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL harus diatur dalam file .env");

    // Membuat pool koneksi dengan maksimal 5 koneksi
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Gagal terhubung ke database")
}
