pub mod app_state;
pub mod cars;
pub mod users;

use dotenvy::dotenv;
use std::env;
use axum::Router;
use http::Method;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let shared_state = app_state::AppState::new().await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    // build our application with a route
    let app = Router::new()
        .nest("/car/", cars::router::car_route())
        .nest("/user/", users::router::user_route())
        .layer(cors)
        .with_state(shared_state);

    // run it
    let server_url = env::var("SERVICE_URL").expect("Service url is not specified");
    let listener = tokio::net::TcpListener::bind(server_url.clone())
        .await
        .unwrap();

    println!("Running on {}", server_url);
    axum::serve(listener, app).await.unwrap();
}
