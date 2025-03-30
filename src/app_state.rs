use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::{Context, Result};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{DateTime, Utc, Duration};
use std::sync::LazyLock;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn new() -> Result<AppState> {
        let connection_string = env::var("DATABASE_URL").expect("Databse url is not specified");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .with_context(|| format!("Error while connecting to {}", connection_string))?;

        Ok(AppState {pool})
    }
}

pub const EXPIRING: Duration = Duration::new(60*60*24*7, 0).unwrap();

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
