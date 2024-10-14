use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_web::HttpMessage; 
use serde;
// Add this import to use extensions()
use sqlx::PgPool;
use crate::models::notification::{LikeNotification, CommentNotification};
use serde::Serialize;

#[derive(Serialize)]
pub enum Notification {
    Like(LikeNotification),
    Comment(CommentNotification),
}

#[derive(Serialize)]
struct SerializableNotification {
    created_at: String, // Change to the appropriate type
    notification: Notification,
}

pub async fn get_notification_history(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    // Log the request extensions for debugging
    println!("Request Extensions: {:?}", req.extensions());

    // Attempt to retrieve user_id from request extensions
    match req.extensions().get::<i32>() {
        Some(user_id) => {
            println!("Extracted user_id: {:?}", user_id);

            // Use the user_id directly as it's already an i32
            let notifications: Vec<LikeNotification> = sqlx::query_as!(
                LikeNotification,
                "SELECT id, detail, redirect, created_at, user_id, yappin_like_id, by_id FROM like_notifications WHERE user_id = $1 ORDER BY created_at DESC",
                *user_id // Dereference to get the value
            )
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_else(|_| vec![]);

            let comment_notifications: Vec<CommentNotification> = sqlx::query_as!(
                CommentNotification,
                "SELECT id, detail, redirect, created_at, user_id, yappin_comment_id, by_id FROM comment_notifications WHERE user_id = $1 ORDER BY created_at DESC",
                *user_id // Dereference to get the value
            )
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_else(|_| vec![]);

            let combined_notifications: Vec<SerializableNotification> = notifications
                .into_iter()
                .map(|n| SerializableNotification {
                    created_at: n.created_at.to_string(),
                    notification: Notification::Like(n),
                })
                .chain(comment_notifications.into_iter().map(|n| SerializableNotification {
                    created_at: n.created_at.to_string(),
                    notification: Notification::Comment(n),
                }))
                .collect();

            return HttpResponse::Ok().json(serde_json::json!( {
                "status": true,
                "message": "Notifications fetched successfully",
                "data": combined_notifications
            }));
        }
        None => {
            println!("User ID not found in request extensions.");
            return HttpResponse::Unauthorized().body("User not found");
        }
    }
}

pub async fn get_chicken(req: HttpRequest) -> impl Responder {

    match req.extensions().get::<i32>() {
        Some(user_id) => {
            HttpResponse::Ok().json(serde_json::json!({"user_id": user_id}))
        },
        None => {
            HttpResponse::Unauthorized().json(serde_json::json!({"error": "Unauthorized"}))
        }
    }
    // // Log the request extensions for debugging
    // println!("Request Extensions: {:?}", req.extensions());

    // // Attempt to retrieve user_id from request extensions
    // if let Some(user_id) = req.extensions().get::<i32>() {
    //     println!("Extracted user_id: {:?}", user_id);
    //     // Return a successful response if user_id exists
    //     return HttpResponse::Ok().body(format!("User ID exists: {}", user_id));
    // } else {
    //     println!("User ID not found in request extensions.");
    //     return HttpResponse::Unauthorized().body("User not found");
    // }
}

// async fn some_handler(req: HttpRequest) -> impl Responder {
//     match req.extensions().get::<i32>() {
//         Some(user_id) => {
//             HttpResponse::Ok().json(json!({"user_id": user_id}))
//         },
//         None => {
//             HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
//         }
//     }
// }
