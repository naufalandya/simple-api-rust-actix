use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    pub search: String,
    pub page: Option<u32>,
}

pub async fn search(query: web::Query<QueryParams>) -> impl Responder {
    let search_term = &query.search;
    let page = query.page.unwrap_or(1);
    let response = format!("Searching for '{}' on page {}", search_term, page);
    HttpResponse::Ok().body(response)
}
