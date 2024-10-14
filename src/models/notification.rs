use serde::{Deserialize, Serialize, Serializer, Deserializer};
use sqlx::FromRow;
use time::{PrimitiveDateTime, macros::format_description}; // Keep only the macro import

fn serialize_datetime<S>(date: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = date.to_string(); // Format as needed
    serializer.serialize_str(&formatted)
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    
    // Define the format for parsing
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    
    PrimitiveDateTime::parse(&s, &format)
        .map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct LikeNotification {
    pub id: i32,
    pub detail: String,
    pub redirect: Option<String>,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub created_at: PrimitiveDateTime,
    pub user_id: i32,
    pub yappin_like_id: i32,
    pub by_id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CommentNotification {
    pub id: i32,
    pub detail: String,
    pub redirect: Option<String>,
    #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
    pub created_at: PrimitiveDateTime,
    pub user_id: i32,
    pub yappin_comment_id: i32,
    pub by_id: i32,
}
