use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::{Context, Result};
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
