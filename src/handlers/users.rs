// // src/handlers/users.rs

// use actix_web::{web, HttpResponse, Responder, HttpRequest};
// use sqlx::PgPool;

// use crate::models::user::{UserResponse, CreateUser, Claims};

// use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};


// fn decode_jwt(token: &str) -> Result<Claims, actix_web::Error> {
//     let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     println!("Secret: {}", secret);  
//     println!("Token: {}", token);    

//     let validation = Validation::new(Algorithm::HS512);  
//     match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation) {
//         Ok(token_data) => Ok(token_data.claims),
//         Err(e) => {
//             println!("Error: {:?}", e); 
//             Err(actix_web::error::ErrorUnauthorized("Invalid token"))
//         },
//     }
// }


// pub async fn create_user(
//     pool: web::Data<PgPool>,
//     info: web::Json<CreateUser>,
// ) -> impl Responder {
//     // Insert user ke database
//     let result = sqlx::query!(
//         "INSERT INTO public.users (name, age) VALUES ($1, $2) RETURNING id",
//         info.name,
//         info.age
//     )
//     .fetch_one(pool.get_ref())
//     .await;

//     match result {
//         Ok(record) => {
//             let response = UserResponse {
//                 message: format!("User {} berusia {} berhasil dibuat dengan ID {}.", info.name, info.age, record.id),
//                 status: 201,
//             };
//             HttpResponse::Created().json(response)
//         },
//         Err(e) => {
//             eprintln!("Gagal membuat user: {}", e);
//             HttpResponse::InternalServerError().body("Gagal membuat user")
//         }
//     }
// }

// pub async fn get_user(
//     pool: web::Data<PgPool>,
//     path: web::Path<i32>,
// ) -> impl Responder {
//     let user_id = path.into_inner();

//     let result = sqlx::query!(
//         "SELECT id, name, age FROM public.users WHERE id = $1",
//         user_id
//     )
//     .fetch_optional(pool.get_ref())
//     .await;

//     match result {
//         Ok(Some(record)) => {
//             let response = serde_json::json!({
//                 "id": record.id,
//                 "name": record.name,
//                 "age": record.age,
//             });
//             HttpResponse::Ok().json(response)
//         },
//         Ok(None) => HttpResponse::NotFound().body("User tidak ditemukan"),
//         Err(e) => {
//             eprintln!("Gagal mengambil user: {}", e);
//             HttpResponse::InternalServerError().body("Gagal mengambil user")
//         }
//     }
// }

// pub async fn whoami(req: HttpRequest) -> impl Responder {
//     // Get the Authorization header
//     if let Some(auth_header) = req.headers().get("Authorization") {
//         if let Ok(auth_str) = auth_header.to_str() {
//             if auth_str.starts_with("Bearer ") {
//                 let token = auth_str.trim_start_matches("Bearer ");

//                 // Decode the JWT
//                 match decode_jwt(token) {
//                     Ok(claims) => {
//                         // Return the user info from the JWT claims
//                         return HttpResponse::Ok().json(serde_json::json!({
//                             "user_id": claims.sub,
//                             "username": claims.username
//                         }));
//                     }
//                     Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
//                 }
//             }
//         }
//     }
//     HttpResponse::Unauthorized().body("Missing or invalid Authorization header")
// }
