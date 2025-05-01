use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::{Context, Result};
use std::env;

use axum::extract::ws::Message;
use tokio::sync::{Mutex, mpsc::Sender};
use std::sync::Arc;
use std::collections::HashMap;

pub type ViewerMap = Arc<Mutex<
    HashMap<String, (Option<Sender<Message>>, Option<Sender<Message>>)>
>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub viewer_map: ViewerMap
}

impl AppState {
    pub async fn new() -> Result<AppState> {
        let connection_string = env::var("DATABASE_URL").expect("Databse url is not specified");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .with_context(|| format!("Error while connecting to {}", connection_string))?;

        let viewer_map = ViewerMap::new(Mutex::new(HashMap::new()));

        Ok(AppState {pool, viewer_map })
    }
}


