use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform, Service},
    error::Error,
    HttpMessage,
    HttpResponse,
    HttpRequest,
    body::BoxBody,
    Result,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use std::rc::Rc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use crate::models::user::Claims;  // Adjust this import as needed
use serde_json::json;

pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        // Extracting the inner HttpRequest from ServiceRequest
        let req_http: HttpRequest = req.request().clone(); // Clone the inner HttpRequest

        let fut = async move {
            // Accessing the Authorization header
            if let Some(auth_header) = req_http.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = auth_str.trim_start_matches("Bearer ");

                        match decode_jwt(token) {
                            Ok(claims) => {
                                req.extensions_mut().insert(claims.sub); // Attach user_id to request
                                return service.call(req).await; // Call the next service
                            }
                            Err(_) => {
                                let err_response = HttpResponse::Unauthorized()
                                    .content_type("application/json")
                                    .body(json!({
                                        "status": 400,
                                        "error": "Invalid token"
                                    }).to_string());

                                // Return the error response with a valid HttpRequest
                                return Ok(ServiceResponse::new(req_http, err_response));  // Using req_http here
                            }
                        }
                    }
                }
            }

            // Handle the missing or invalid Authorization header
            let err_response = HttpResponse::Unauthorized()
                .content_type("application/json")
                .body(json!({
                    "status": 400,
                    "error": "Missing or invalid Authorization header"
                }).to_string());

            // Return the error response with a valid HttpRequest
            Ok(ServiceResponse::new(req_http, err_response))
        };

        Box::pin(fut)
    }
}

fn decode_jwt(token: &str) -> Result<Claims, actix_web::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let validation = Validation::new(Algorithm::HS512);
    match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}