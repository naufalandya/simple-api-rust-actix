// src/models/user.rs

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
    pub status: u16,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub age: u8,
}

#[derive(Deserialize)]
pub struct GetUser {
    pub id: i32,
}