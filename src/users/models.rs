use serde::{Serialize, Deserialize};
use sqlx;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
 
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct User {
    #[serde(skip_serializing)]
    pub pk: i32,

    pub id: Uuid,

    #[serde(with = "time::serde::rfc3339")]
    pub create_time: OffsetDateTime,
    pub username: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub is_super: bool
}

#[derive(Debug, serde::Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub is_super: Option<bool>
}

#[derive(Debug, serde::Deserialize)]
pub struct UserDelete {
    pub id: Uuid
}

#[derive(Debug, serde::Deserialize)]
pub struct UserUpdate {
    pub id: Uuid,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_super: Option<bool>
}
