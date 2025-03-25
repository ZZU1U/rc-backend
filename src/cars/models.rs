use sqlx;
use std::net::IpAddr;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Car {
    pub pk: i32,
    pub id: Uuid,
    pub create_time: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub key_hash: String,
    pub creator_id: Uuid,
    pub ip: Option<IpAddr>
}

#[derive(Debug, serde::Deserialize)]
pub struct CarCreate {
    pub name: String,
    pub description: Option<String>,
    pub key: String,
}
