use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform, Service},
    error::Error,
    HttpMessage,
    HttpResponse,
    body::BoxBody,
    Result,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::task::{Context, Poll};
use std::rc::Rc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use crate::models::user::Claims; 
use serde_json::json;

// Define the AuthMiddleware struct
pub struct AuthMiddleware;

// Implement the Transform trait for AuthMiddleware
impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    // Define the response type
    type Response = ServiceResponse<BoxBody>;
    // Define the error type
    type Error = Error;
    // Define the middleware's transform type
    type Transform = AuthMiddlewareMiddleware<S>;
    // Define the initialization error type
    type InitError = ();
    // Define the future type for new_transform
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // Initialize the middleware transform
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

// Define the middleware struct that will handle requests
pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

// Implement the Service trait for AuthMiddlewareMiddleware
impl<S> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    // Define the response type
    type Response = ServiceResponse<BoxBody>;
    // Define the error type
    type Error = Error;
    // Define the future type for call
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Delegate the readiness check to the inner service
    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    // Handle the request
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        // Clone the Authorization header to avoid holding onto HttpRequest
        let auth_header = req.headers().get("Authorization").cloned();

        Box::pin(async move {
            if let Some(auth_header) = auth_header {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = auth_str.trim_start_matches("Bearer ");
                        match decode_jwt(token) {
                            Ok(claims) => {
                                println!("Claims: {:?}", claims);

                                // Insert the user_id into request extensions
                                req.extensions_mut().insert(claims.id);

                                // Call the next service and return its response
                                let res = service.call(req).await?;
                                return Ok(res);
                            }
                            Err(_) => {
                                return Ok(ServiceResponse::new(
                                    req.into_parts().0, // Extract the request
                                    HttpResponse::Unauthorized()
                                        .content_type("application/json")
                                        .body(json!({"status": 401, "error": "Invalid token"}).to_string()),
                                ));
                            }
                        }
                    }
                }
            }

            // If Authorization header is missing or invalid
            Ok(ServiceResponse::new(
                req.into_parts().0, // Extract the request
                HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .body(json!({"status": 401, "error": "Missing or invalid Authorization header"}).to_string()),
            ))
        })
    }
}

// Function to decode JWT tokens
fn decode_jwt(token: &str) -> Result<Claims, actix_web::Error> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let validation = Validation::new(Algorithm::HS512);

    match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => {
            println!("JWT Decode Error: {:?}", e); 
            Err(actix_web::error::ErrorUnauthorized("Invalid token"))
        }
    }
}
