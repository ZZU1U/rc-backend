pub mod app_state;
pub mod cars;
pub mod users;

use dotenvy::dotenv;
use std::env;
use axum::Router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let shared_state = app_state::AppState::new().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .nest("/user/", users::router::user_route())
        .nest("/car/", cars::router::car_route())
        .with_state(shared_state);

    // run it
    let server_url = env::var("SERVICE_URL").expect("Service url is not specified");
    let listener = tokio::net::TcpListener::bind(server_url.clone())
        .await
        .unwrap();

    println!("Running on {}", server_url);
    axum::serve(listener, app).await.unwrap();
}
