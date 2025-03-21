pub mod app_state;
pub mod cars;
pub mod users;

use dotenv::dotenv;
use std::env;
use axum::{response::{Html, Json}, routing::get, Router, extract::State};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let share_state = app_state::AppState::new().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .with_state(share_state)
        .merge(cars::router::cars_router())
        .merge(users::router::register_route());

    // run it
    let server_url = env::var("SERVICE_URL").expect("Service url is not specified");
    let listener = tokio::net::TcpListener::bind(server_url.clone())
        .await
        .unwrap();

    println!("Running on {}", server_url);
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(state): State<app_state::AppState>) -> Html<String> {
    let result: Result<String, _> = sqlx::query_scalar(
        r#"
        SELECT version();
        "#
    ).fetch_one(&state.pool).await;

    match result {
        Ok(version) => {
            Html(version)
        },
        Err(_e) => Html(String::new())
    }
}
