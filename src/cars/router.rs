use axum::{
    routing::{get, post, MethodRouter},
    response::{Html, Json}, 
    Router,
    extract::State, 
    http::StatusCode, 
};
use uuid::Uuid;
use crate::app_state::AppState;
use crate::cars::models::{Car, CarCreate};
use crate::users::passwords::hash_password;
use crate::users::auth::models::Claims;

pub fn car_route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_cars))
        .route("/", post(create_car))
}

async fn create_car(claims: Claims, State(state): State<AppState>, car: Json<CarCreate>) -> Result<(StatusCode, Json<Uuid>), StatusCode> {
    if !claims.is_super.unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }

    let password_hash = hash_password(car.key.clone()).await.unwrap();

    let result = sqlx::query!(
        r#"
        INSERT INTO car (name, description, key_hash, creator_id, image_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        car.name, car.description, password_hash, claims.sub, car.image_url
    )  
        .fetch_one(&state.pool).await.unwrap();

    println!("{:?}", result);

    return Ok((StatusCode::OK, Json(result.id)));
}

async fn get_cars(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<Car>>), StatusCode>  {
    let result = sqlx::query_as!(Car,
        r#"
        SELECT * FROM car
        "#,
    )
        .fetch_all(&state.pool).await.unwrap();

    Ok((StatusCode::OK, Json(result)))
}

