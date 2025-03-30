use axum::{
    response::Json,
    routing::{get, post}, 
    Router, 
    extract::State, 
    http::StatusCode, 
};
use uuid::Uuid;
use crate::users::models::{User, UserCreate};
use crate::users::passwords::{hash_password, check_password};
use crate::app_state::AppState;
use crate::users::auth::{
    models::{Claims, AuthError, AuthBody, AuthPayload}, 
    utils::create_token
};

pub fn user_route() -> Router<AppState> {
    Router::new()
        .route("/", post(register))
        .route("/", get(get_all))
        .route("/jwt/", post(login))
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
        user.username, user.is_super.unwrap_or(false), password_hash
    )  
        .fetch_one(&state.pool).await.unwrap();

    return Ok((StatusCode::OK, Json(result.id)));
}

async fn login(State(state): State<AppState>, Json(payload): Json<AuthPayload>) -> Result<(StatusCode, Json<AuthBody>), StatusCode> {
    let result = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM "user"
        WHERE "user".username = $1
        "#,
        payload.username
    )  
        .fetch_one(&state.pool).await.unwrap();

    match check_password(result.password_hash.clone(), payload.password.clone()).await.unwrap() {
        true => {
            let token = create_token(&result)
                .await.unwrap();
            Ok((StatusCode::OK, Json(AuthBody::new(token))))
        },
        false => Err(StatusCode::UNAUTHORIZED)
    }
}

async fn get_all(claims: Claims, State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    if !claims.is_super.unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM "user"
        "#,
    )  
        .fetch_all(&state.pool).await.unwrap();

    return Ok((StatusCode::OK, Json(result)))
}

async fn protected(_: Claims) -> Result<StatusCode, AuthError> {
    Ok(StatusCode::OK)
}
