// src/main.rs

use actix_web::{App, HttpServer};
use env_logger;
use dotenv::dotenv;

mod handlers;
mod models;
mod routes;
mod responses;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Memuat file .env
    env_logger::init(); // Inisialisasi logger

    // Inisialisasi pool koneksi database
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default()) // Middleware Logger
            .app_data(actix_web::web::Data::new(pool.clone())) // Menambahkan pool ke data aplikasi
            .configure(routes::init_routes) // Mengatur semua route
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
