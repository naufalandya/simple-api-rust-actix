use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_web::HttpMessage; // Add this import to use extensions()
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
    if let Some(user_id) = req.extensions().get::<String>() {
        let user_id = user_id.parse::<i32>().unwrap_or(0);

        if user_id == 0 {
            return HttpResponse::BadRequest().json(serde_json::json!( {
                "status": false,
                "message": "User ID is required",
                "data": null
            }));
        }
        
        let notifications: Vec<LikeNotification> = sqlx::query_as!(
            LikeNotification,
            "SELECT id, detail, redirect, created_at, user_id, yappin_like_id, by_id FROM like_notifications WHERE user_id = $1 ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_else(|_| vec![]);

        let comment_notifications: Vec<CommentNotification> = sqlx::query_as!(
            CommentNotification,
            "SELECT id, detail, redirect, created_at, user_id, yappin_comment_id, by_id FROM comment_notifications WHERE user_id = $1 ORDER BY created_at DESC",
            user_id
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


        // combined_notifications.sort_by(|a, b| b.0.cmp(&a.0));

        return HttpResponse::Ok().json(serde_json::json!( {
            "status": true,
            "message": "Notifications fetched successfully",
            "data": combined_notifications
        }));
    }

    HttpResponse::Unauthorized().body("User not found")
}
