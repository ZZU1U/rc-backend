use sqlx;
use uuid::Uuid;
use sqlx::types::time::OffsetDateTime;

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Car {
    #[serde(skip_serializing)]
    pub id: Uuid,

    #[serde(with = "time::serde::rfc3339")]
    pub last_seen: OffsetDateTime,

    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    //pub is_on: bool, // todo

    #[serde(skip_serializing)]
    pub key_hash: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CarCreate {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CarDelete {
    pub id: Uuid
}

#[derive(Debug, serde::Deserialize)]
pub struct CarUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}
