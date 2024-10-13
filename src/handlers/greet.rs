use actix_web::{web, HttpResponse, Responder};

pub async fn greet(path: web::Path<(String,)>) -> impl Responder {
    let name = path.into_inner().0;
    let greeting = format!("Hello, {}!", name);
    HttpResponse::Ok().body(greeting)
}
