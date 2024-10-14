// src/routes/mod.rs

use actix_web::web;


use crate::handlers::notification::get_notification_history;
use crate::middlewares::auth_middleware::AuthMiddleware;

// use crate::handlers::{
//     greet::greet,
//     search::search,
//     // users::{create_user, get_user, whoami},
//     error::custom_response,
// };

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/api")
            .wrap(AuthMiddleware)  // Apply the middleware
            .route("/notifications", web::get().to(get_notification_history))
    );
        // Route dengan path parameter
        // .route("/greet/{name}", web::get().to(greet))
        
        // Route dengan query parameter
        // .route("/search", web::get().to(search))
        
        // Route untuk menerima JSON dan mengirimkan JSON dengan status code
        // .route("/users", web::post().to(create_user))
        
        // Route untuk mengembalikan JSON berdasarkan ID
        // .route("/users/{id}", web::get().to(get_user))
        
        // Route untuk mengembalikan text dengan status code khusus
        // .route("/error", web::get().to(custom_response))

        // .route("/who-i-am", web::get().to(whoami)); // New route for whoami

}
