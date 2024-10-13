use actix_web::{http::StatusCode, HttpResponse, Responder};

pub async fn custom_response() -> impl Responder {
    HttpResponse::build(StatusCode::BAD_REQUEST)
        .body("Permintaan tidak valid.")
}
