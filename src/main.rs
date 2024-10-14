// src/main.rs

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors; // Import crate actix-cors
use env_logger;
use dotenv::dotenv;

mod handlers;
mod models;
mod routes;
mod responses;
mod db;
mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Memuat file .env
    env_logger::init(); // Inisialisasi logger

    // Inisialisasi pool koneksi database
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        // Konfigurasi CORS
        let cors = Cors::default()
            .allow_any_origin() // Mengizinkan semua origin. Untuk produksi, sebaiknya tentukan origin spesifik menggunakan `.allowed_origin("http://example.com")`
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]) // Metode HTTP yang diizinkan
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ]) // Header yang diizinkan
            .max_age(3600); // Durasi cache preflight request dalam detik

        App::new()
            .wrap(cors) // Menambahkan middleware CORS
            .wrap(Logger::default()) // Middleware Logger
            .app_data(actix_web::web::Data::new(pool.clone())) // Menambahkan pool ke data aplikasi
            .configure(routes::init_routes) // Mengatur semua route
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
