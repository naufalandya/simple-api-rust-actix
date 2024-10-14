// src/routes/mod.rs

use actix_web::web;


use crate::handlers::notification::{get_notification_history, get_chicken};
use crate::middlewares::auth_middleware::AuthMiddleware;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/api")
            .wrap(AuthMiddleware)  // Apply the middleware
            .route("/notifications", web::get().to(get_notification_history))
            .route("/chicken", web::get().to(get_chicken))
    );

}
