// src/models/user.rs

use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// pub struct Info {
//     pub name: String,
//     pub age: u8,
// }

#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
    pub status: u16,
}

// src/models/user.rs
#[derive(serde::Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub age: i32, // Change this to i32
}


// #[derive(Deserialize)]
// pub struct GetUser {
//     pub id: i32,
// }