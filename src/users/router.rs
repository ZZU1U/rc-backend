use axum::{
    response::{Html, Json}, 
    routing::{get, post}, 
    Router, 
    extract::State, 
    http::StatusCode, 
};
use uuid::Uuid;
use crate::users::models::{User, UserCreate, UserLogin};
use crate::users::passwords::{hash_password, check_password};
use crate::users::auth::{JWT, create_token, Claims, AuthError};
use crate::app_state::AppState;

pub fn user_route() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/all", get(get_all))
        .route("/login", post(login))
        .route("/check", get(protected))
}

async fn register(State(state): State<AppState>, user: Json<UserCreate>) -> Result<(StatusCode, Json<Uuid>), StatusCode> {
    let password_hash = hash_password(user.password.clone()).await.unwrap();

    let result = sqlx::query!(
        r#"
        INSERT INTO "user" (username, is_super, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        user.username, user.is_super, password_hash
    )  
        .fetch_one(&state.pool).await.unwrap();

    return Ok((StatusCode::OK, Json(result.id)));
}

async fn login(State(state): State<AppState>, user: Json<UserLogin>) -> Result<(StatusCode, Json<JWT>), StatusCode> {
    let result = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM "user"
        WHERE "user".username = $1
        "#,
        user.username
    )  
        .fetch_one(&state.pool).await.unwrap();

    match check_password(result.password_hash.clone(), user.password.clone()).await.unwrap() {
        true => Ok((StatusCode::OK, Json(
                    JWT { token: create_token(&result) }
                    ))),
        false => Err(StatusCode::UNAUTHORIZED)
    }
}

async fn get_all(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let result = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM "user"
        "#,
    )  
        .fetch_all(&state.pool).await.unwrap();

    return Ok((StatusCode::OK, Json(result)))
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{:?}", claims
    ))
}
