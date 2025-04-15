use serde::Serialize;
use sqlx;
use uuid::Uuid;
 
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct User {
    pub id: Uuid,

    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub is_super: bool,
    pub is_verified: bool
}

#[derive(Debug, serde::Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_super: Option<bool>,
    pub is_verified: Option<bool>
}

#[derive(Debug, serde::Deserialize)]
pub struct UserDelete {
    pub id: Option<Uuid>
}

#[derive(Debug, serde::Deserialize)]
pub struct UserUpdate {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    //pub password: Option<String>,
    pub is_super: Option<bool>,
    pub is_verified: Option<bool>
}
