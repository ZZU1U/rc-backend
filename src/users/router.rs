use axum::{
    response::Json,
    routing::{get, post, put, delete}, 
    Router, 
    extract::State, 
    http::StatusCode, 
};
use uuid::Uuid;
use crate::app_state::AppState;
use super::models::{User, UserCreate, UserUpdate, UserDelete};
use super::passwords::{hash_password, check_password};
use super::auth::{
    models::{Claims, AuthBody, UserAuthPayload}, 
    utils::generate_token
};

pub fn user_route() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user))
        .route("/", put(update_info))
        .route("/", get(get_users))
        .route("/", delete(delete_user))
        .route("/jwt", post(create_token))
        //.route("/check", get(protected))
}

async fn create_user(State(state): State<AppState>, data: Json<UserCreate>) -> Result<(StatusCode, Json<User>), StatusCode> {
    let password_hash = hash_password(data.password.clone()).await.unwrap();

    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO "user" (id, is_super, password_hash, email)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        Uuid::now_v7(), data.is_super.unwrap_or(false), 
        password_hash, data.email
    )  
        .fetch_one(&state.pool).await;

    let user;
    match result {
        Ok(res) => user = res,
        Err(_) => return Err(StatusCode::CONFLICT)
    };

    Ok((
        StatusCode::OK, 
        Json(user)
    ))
}

async fn create_token(State(state): State<AppState>, Json(payload): Json<UserAuthPayload>) -> Result<(StatusCode, Json<AuthBody>), StatusCode> {
    println!("{:?}", payload);

    let result = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM "user"
        WHERE "user".email = $1
        "#,
        payload.email
    )  
        .fetch_one(&state.pool).await;

    let user;
    match result {
        Ok(res) => user = res,
        Err(_) => return Err(StatusCode::UNAUTHORIZED)
    }

    if !user.is_verified {
        return Err(StatusCode::UNAUTHORIZED);
    }

    match check_password(user.password_hash.clone(), payload.password.clone()).await.unwrap() {
        true => {
            let claims = Claims::new(&user);
            let token = generate_token(claims)
                .await.unwrap();
            Ok((StatusCode::OK, Json(AuthBody::new(token))))
        },
        false => Err(StatusCode::UNAUTHORIZED)
    }
}

async fn get_users(claims: Claims, State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    if !claims.is_super.unwrap_or(false) {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query_as!(
        User,
        r#"
        SELECT * 
        FROM "user"
        ORDER BY is_verified ASC
        "#,
    )  
        .fetch_all(&state.pool).await.unwrap();

    Ok((StatusCode::OK, Json(result)))
}

async fn update_info(claims: Claims, State(state): State<AppState>, mut data: Json<UserUpdate>) -> Result<(StatusCode, Json<User>), StatusCode> {
    data.id = Some(data.id.unwrap_or(claims.sub));

    if (!claims.is_super.unwrap_or(false)) && ((claims.sub != data.id.unwrap()) || (data.is_verified.unwrap_or(false)) || (data.is_super.unwrap_or(false))) {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query_as!(
        User,
        r#"
        UPDATE "user"
        SET email = COALESCE($1, email), is_super = COALESCE($2, is_super), is_verified = COALESCE($3, is_verified)
        WHERE id = $4
        RETURNING *
        "#,
        data.email, data.is_super, data.is_verified, data.id
    )
        .fetch_one(&state.pool).await;

    let user;
    match result {
        Ok(res) => user = res,
        Err(_) => return Err(StatusCode::FORBIDDEN)
    }

    Ok((StatusCode::OK, Json(user)))
}

async fn delete_user(claims: Claims, State(state): State<AppState>, mut data: Json<UserDelete>) -> Result<(StatusCode, Json<User>), StatusCode> {
    data.id = Some(data.id.unwrap_or(claims.sub));

    if (!claims.is_super.unwrap_or(false)) && (claims.sub != data.id.unwrap()) {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query_as!(
        User,
        r#"
        DELETE FROM "user"
        WHERE id = $1
        RETURNING *
        "#,
        data.id.unwrap()
    )
        .fetch_one(&state.pool).await;

    let user;
    match result {
        Ok(res) => user = res,
        Err(_) => return Err(StatusCode::FORBIDDEN)
    }

    return Ok((StatusCode::OK, Json(user)))
}

// async fn protected(claims: Claims) -> Result<(StatusCode, Json<Claims>), AuthError> { 
//     Ok((StatusCode::OK, Json(claims)))
// }
