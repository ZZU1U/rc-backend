use axum::{
    routing::{get, post, put, delete},
    response::Json, 
    Router,
    extract::State, 
    http::StatusCode, 
};
use uuid::Uuid;
use crate::{app_state::AppState, users::auth::models::{AuthBody, TokenType}};
use crate::users::passwords::{hash_password, check_password};
use crate::users::auth::{models::{Claims, CarAuthPayload}, utils::generate_token};
use super::models::{Car, CarCreate, CarDelete, CarUpdate};

pub fn car_route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_cars))
        .route("/", post(create_car))
        .route("/", put(update_car))
        .route("/", delete(delete_car))
        .route("/jwt", post(create_token))
}

async fn create_car(claims: Claims, State(state): State<AppState>, car: Json<CarCreate>) -> Result<(StatusCode, Json<Car>), StatusCode> {
    if !claims.is_super.unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }

    let password_hash = hash_password(car.key.clone()).await.unwrap();

    let result = sqlx::query_as!(
        Car,
        r#"
        INSERT INTO car (id, name, description, key_hash, image_url, power)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        Uuid::now_v7(), car.name, car.description, password_hash, car.image_url, car.power
    )  
        .fetch_one(&state.pool).await.unwrap();

    return Ok((StatusCode::OK, Json(result)));
}

async fn get_cars(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<Car>>), StatusCode>  {
    let result = sqlx::query_as!(Car,
        r#"
        SELECT * 
        FROM car
        ORDER BY is_on DESC
        "#,
    )
        .fetch_all(&state.pool).await.unwrap();

    Ok((StatusCode::OK, Json(result)))
}

async fn update_car(claims: Claims, State(state): State<AppState>, data: Json<CarUpdate>) -> Result<(StatusCode, Json<Car>), StatusCode> {
    if !(claims.is_super.unwrap_or(false) || matches!(claims.token_type, TokenType::Service)) {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query_as!(
        Car,
        r#"
        UPDATE car
        SET name = COALESCE($1, name), description = COALESCE($2, description),
        image_url = COALESCE($3, image_url), is_on = COALESCE($4, is_on), power = COALESCE($5, power)
        WHERE id = $6
        RETURNING *
        "#,
        data.name, data.description, data.image_url, data.is_on, data.power, data.id
    )
        .fetch_one(&state.pool).await;

    let car;
    match result {
        Ok(res) => car = res,
        Err(_) => return Err(StatusCode::FORBIDDEN)
    }

    Ok((StatusCode::OK, Json(car)))
}

async fn delete_car(claims: Claims, State(state): State<AppState>, data: Json<CarDelete>) -> Result<(StatusCode, Json<Car>), StatusCode> {
    if !claims.is_super.unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN)
    }

    let result = sqlx::query_as!(
        Car,
        r#"
        DELETE FROM car
        WHERE id = $1
        RETURNING *
        "#,
        data.id
    )
        .fetch_one(&state.pool).await;

    let car;
    match result {
        Ok(res) => car = res,
        Err(_) => return Err(StatusCode::FORBIDDEN)
    }

    return Ok((StatusCode::OK, Json(car)))
}

async fn create_token(State(state): State<AppState>, payload: Json<CarAuthPayload>) -> Result<(StatusCode, Json<AuthBody>), StatusCode> {
    let result = sqlx::query_as!(
        Car,
        r#"
        SELECT * FROM car
        WHERE id = $1
        "#,
        payload.id
    )  
        .fetch_one(&state.pool).await;

    let car;
    match result {
        Ok(res) => car = res,
        Err(_) => return Err(StatusCode::UNAUTHORIZED)
    }

    match check_password(car.key_hash.clone(), payload.key.clone()).await.unwrap() {
        true => {
            let claims = Claims::new(&car);
            let token = generate_token(claims)
                .await.unwrap();
            Ok((StatusCode::OK, Json(AuthBody::new(token))))
        },
        false => Err(StatusCode::UNAUTHORIZED)
    }
}
