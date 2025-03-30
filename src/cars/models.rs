use sqlx;
use uuid::Uuid;
use sqlx::types::{ipnetwork::IpNetwork, time::OffsetDateTime};

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Car {
    #[serde(skip_serializing)]
    pub pk: i32,

    pub id: Uuid,
    pub create_time: OffsetDateTime,
    pub name: String,
    pub description: Option<String>,

    #[serde(skip_serializing)]
    pub key_hash: String,

    #[serde(skip_serializing)]
    pub creator_id: Uuid,

    pub ip: Option<IpNetwork>
}

#[derive(Debug, serde::Deserialize)]
pub struct CarCreate {
    pub name: String,
    pub description: Option<String>,
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
    pub ip: Option<IpNetwork>
}
