use actix_web::{web, HttpResponse, Responder, HttpRequest};
use actix_web::HttpMessage;
use sqlx::PgPool;
use crate::models::notification::{LikeNotification, CommentNotification};
use futures::future;
use std::collections::{HashMap, HashSet};

pub async fn get_notification_history(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    println!("Request Extensions: {:?}", req.extensions());

    // Attempt to retrieve user_id from request extensions
    match req.extensions().get::<i32>() {
        Some(user_id) => {
            println!("Extracted user_id: {:?}", user_id);

            // Fetch like and comment notifications concurrently
            let notifications_future = future::try_join(
                sqlx::query_as!(
                    LikeNotification,
                    "SELECT id, detail, redirect, created_at, user_id, yappin_like_id, by_id FROM like_notifications WHERE user_id = $1 ORDER BY created_at DESC",
                    *user_id
                )
                .fetch_all(pool.get_ref()),
                sqlx::query_as!(
                    CommentNotification,
                    "SELECT id, detail, redirect, created_at, user_id, yappin_comment_id, by_id FROM comment_notifications WHERE user_id = $1 ORDER BY created_at DESC",
                    *user_id
                )
                .fetch_all(pool.get_ref()),
            );

            // Await both notifications concurrently and handle potential errors
            let (like_notifications, comment_notifications) = match notifications_future.await {
                Ok((likes, comments)) => (likes, comments),
                Err(e) => {
                    println!("Error fetching notifications: {:?}", e);
                    return HttpResponse::InternalServerError().body("Error fetching notifications");
                }
            };

            // Collect all unique by_ids from both like and comment notifications
            let mut user_ids_set = HashSet::new();
            for notification in &like_notifications {
                user_ids_set.insert(notification.by_id);
            }
            for notification in &comment_notifications {
                user_ids_set.insert(notification.by_id);
            }
            let user_ids: Vec<i32> = user_ids_set.into_iter().collect();

            // Fetch all relevant users in a single query
            let users = if !user_ids.is_empty() {
                match get_users_by_ids(pool.get_ref(), &user_ids).await {
                    Ok(user_list) => user_list,
                    Err(e) => {
                        println!("Error fetching users: {:?}", e);
                        vec![]
                    }
                }
            } else {
                vec![]
            };

            // Create a map for quick user lookup
            let user_map: HashMap<i32, User> = users.into_iter().map(|u| (u.id, u)).collect();

            // Initialize a vector to hold all combined notifications
            let mut combined_notifications = Vec::with_capacity(like_notifications.len() + comment_notifications.len());

            // Process like notifications
            for like in like_notifications {
                let user = user_map.get(&like.by_id);
                combined_notifications.push(serde_json::json!({
                    "id": like.id,
                    "detail": like.detail,
                    "redirect": like.redirect,
                    "created_at": like.created_at.to_string(),
                    "yappin_like_id": like.yappin_like_id,
                    "byusers": {
                        "username": user.map(|u| u.username.clone()).unwrap_or_default(),
                        "avatar_link": user.and_then(|u| u.avatar_link.clone()).unwrap_or_default(),
                    }
                }));
            }

            // Process comment notifications
            for comment in comment_notifications {
                let user = user_map.get(&comment.by_id);
                combined_notifications.push(serde_json::json!({
                    "id": comment.id,
                    "detail": comment.detail,
                    "redirect": comment.redirect,
                    "created_at": comment.created_at.to_string(),
                    "yappin_comment_id": comment.yappin_comment_id,
                    "byusers": {
                        "username": user.map(|u| u.username.clone()).unwrap_or_default(),
                        "avatar_link": user.and_then(|u| u.avatar_link.clone()).unwrap_or_default(),
                    }
                }));
            }

            // Optionally, sort the combined notifications by created_at descending
            combined_notifications.sort_by(|a, b| {
                let a_time = a["created_at"].as_str().unwrap_or("");
                let b_time = b["created_at"].as_str().unwrap_or("");
                b_time.cmp(a_time)
            });

            // Respond with the combined notifications
            HttpResponse::Ok().json(serde_json::json!({
                "status": true,
                "message": "Notifications fetched successfully",
                "data": combined_notifications
            }))
        }
        None => {
            println!("User ID not found in request extensions.");
            HttpResponse::Unauthorized().body("User not found")
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_link: Option<String>,
}

// Async function to get users by IDs in a single query
async fn get_users_by_ids(pool: &PgPool, user_ids: &[i32]) -> Result<Vec<User>, sqlx::Error> {
    if user_ids.is_empty() {
        return Ok(vec![]);
    }

    sqlx::query_as!(
        User,
        "SELECT id, username, avatar_link FROM users WHERE id = ANY($1)",
        user_ids
    )
    .fetch_all(pool)
    .await
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


//lol

// use actix_web::{web, HttpResponse, Responder, HttpRequest};
// use actix_web::HttpMessage;
// use sqlx::PgPool;
// use crate::models::notification::{LikeNotification, CommentNotification};
// use futures::future; 

// pub async fn get_notification_history(
//     pool: web::Data<PgPool>, // Keep as is
//     req: HttpRequest,
// ) -> impl Responder {
//     // Log the request extensions for debugging
//     println!("Request Extensions: {:?}", req.extensions());

//     // Attempt to retrieve user_id from request extensions
//     match req.extensions().get::<i32>() {
//         Some(user_id) => {
//             println!("Extracted user_id: {:?}", user_id);

//             // Fetch like notifications
//             let notifications: Vec<LikeNotification> = sqlx::query_as!(
//                 LikeNotification,
//                 "SELECT id, detail, redirect, created_at, user_id, yappin_like_id, by_id FROM like_notifications WHERE user_id = $1 ORDER BY created_at DESC",
//                 *user_id // Dereference to get the value
//             )
//             .fetch_all(pool.get_ref())
//             .await
//             .unwrap_or_else(|_| vec![]);

//             // Fetch comment notifications
//             let comment_notifications: Vec<CommentNotification> = sqlx::query_as!(
//                 CommentNotification,
//                 "SELECT id, detail, redirect, created_at, user_id, yappin_comment_id, by_id FROM comment_notifications WHERE user_id = $1 ORDER BY created_at DESC",
//                 *user_id // Dereference to get the value
//             )
//             .fetch_all(pool.get_ref())
//             .await
//             .unwrap_or_else(|_| vec![]);

//             // Prepare futures for user lookups
//             let mut user_futures = Vec::new();

//             // Collect futures for like notifications
//             for n in &notifications {
//                 let user_id = n.by_id; // Get user id from notification
//                 let future = get_user_by_id(pool.get_ref(), user_id); // Directly pass pool.get_ref()
//                 user_futures.push(future);
//             }

//             // Collect futures for comment notifications
//             for n in &comment_notifications {
//                 let user_id = n.by_id; // Get user id from notification
//                 let future = get_user_by_id(pool.get_ref(), user_id); // Directly pass pool.get_ref()
//                 user_futures.push(future);
//             }

//             // Await all user lookups concurrently
//             let users = future::join_all(user_futures).await;

//             // Combine notifications into a structured response
//             let combined_notifications: Vec<serde_json::Value> = notifications
//             .into_iter()
//             .zip(users.iter()) // Combine notifications with user details
//             .map(|(n, user)| {
//                 let username = user.as_ref().map(|u| u.username.clone()).unwrap_or_default(); // Handle None
//                 let avatar_link = user.as_ref().and_then(|u| u.avatar_link.clone()).unwrap_or_default(); // Handle None
//                 serde_json::json!( {
//                     "id": n.id,
//                     "detail": n.detail,
//                     "redirect": n.redirect,
//                     "created_at": n.created_at.to_string(),
//                     "yappin_like_id": n.yappin_like_id,
//                     "byusers": {
//                         "username": username,
//                         "avatar_link": avatar_link,
//                     }
//                 })
//             })
//             .chain(comment_notifications.into_iter().zip(users.iter()).map(|(n, user)| {
//                 let username = user.as_ref().map(|u| u.username.clone()).unwrap_or_default(); // Handle None
//                 let avatar_link = user.as_ref().and_then(|u| u.avatar_link.clone()).unwrap_or_default(); // Handle None
//                 serde_json::json!( {
//                     "id": n.id,
//                     "detail": n.detail,
//                     "redirect": n.redirect,
//                     "created_at": n.created_at.to_string(),
//                     "yappin_comment_id": n.yappin_comment_id,
//                     "byusers": {
//                         "username": username,
//                         "avatar_link": avatar_link,
//                     }
//                 })
//             }))
//             .collect();

//             return HttpResponse::Ok().json(serde_json::json!( {
//                 "status": true,
//                 "message": "Notifications fetched successfully",
//                 "data": combined_notifications
//             }));
//         }
//         None => {
//             println!("User ID not found in request extensions.");
//             return HttpResponse::Unauthorized().body("User not found");
//         }
//     }
// }

// #[allow(dead_code)]
// #[derive(Debug)]
// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub avatar_link: Option<String>, // Include other fields as needed
// }

// // Async function to get user by ID
// async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Option<User> {
//     // Query to fetch the user by ID
//     let user = sqlx::query_as!(
//         User,
//         "SELECT id, username, avatar_link FROM users WHERE id = $1",
//         user_id
//     )
//     .fetch_one(pool)
//     .await
//     .ok(); // Return None if the user is not found

//     user
// }
