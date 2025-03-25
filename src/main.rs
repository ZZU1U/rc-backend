pub mod app_state;
pub mod cars;
pub mod users;

use crate::cars::models::{Car, CarCreate};
use crate::users::passwords::hash_password;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use axum::{response::{Html, Json}, routing::{get, post}, Router, extract::State, http::StatusCode, response::IntoResponse};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let share_state = app_state::AppState::new().await.unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(get_all))
        .route("/", post(insert_new))
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

async fn get_all(State(state): State<app_state::AppState>) -> Result<(StatusCode, Json<Vec<Car>>), StatusCode> {
    let result = sqlx::query_as::<_, Car>(
        " SELECT * FROM car "
    ).fetch_all(&state.pool).await;

    println!("{:?}", result);

    match result {
        Ok(cars) => Ok((StatusCode::OK, Json(cars))),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn insert_new(State(state): State<app_state::AppState>, car: Json<CarCreate>) -> Result<(StatusCode, Json<Uuid>), StatusCode> {
    let password_hash = hash_password(car.key.clone()).await.unwrap();

    let result = sqlx::query!(
        r#"
        INSERT INTO car (name, description, key_hash, creator_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        car.name, car.description, password_hash, Uuid::parse_str("95022733-f013-301a-0ada-abc18f151006").unwrap()
    )  
        .fetch_one(&state.pool).await.unwrap();

    println!("{:?}", result);

    return Ok((StatusCode::OK, Json(result.id.expect("Huh"))));
}
