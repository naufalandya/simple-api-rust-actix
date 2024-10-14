// src/models/user.rs

use serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
    pub status: u16,
}

// #[derive(Deserialize)]
// pub struct CreateUser {
//     pub name: String,
//     pub age: i32, // Change this to i32
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id : i32,
    pub sub: String,  // User ID (subject)
    pub username: String,
    pub exp: usize,   // Expiry time
}
