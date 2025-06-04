use sqlx;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Car {
    pub id: Uuid,

    #[serde(with = "time::serde::rfc3339")]
    pub last_seen: OffsetDateTime,

    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub power: Option<i32>,
    pub is_on: bool,

    #[serde(skip_serializing)]
    pub key_hash: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CarCreate {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub power: Option<i32>,
    pub key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CarDelete {
    pub id: Uuid,
}

#[derive(Debug, serde::Deserialize)]
pub struct CarUpdate {
    pub id: Uuid,
    pub is_on: Option<bool>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub power: Option<i32>,
    pub image_url: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_seen: Option<OffsetDateTime>,
}
