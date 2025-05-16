use crate::cars::models::Car;
use crate::users::auth::vars::{EXPIRING, KEYS};
use crate::users::models::User;
use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use chrono::Utc;
use jsonwebtoken::{Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "Car")]
    Car,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Service")]
    Service,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub token_type: TokenType,
    pub email: Option<String>,
    pub exp: i64,
    pub is_super: Option<bool>,
    pub iat: i64,
}

impl Claims {
    pub fn new<T>(args: T) -> Claims
    where
        T: Into<Claims>,
    {
        args.into()
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl From<&User> for Claims {
    fn from(user: &User) -> Claims {
        let time = Utc::now().timestamp();
        Claims {
            token_type: TokenType::User,
            sub: user.id,
            email: Some(user.email.clone()),
            exp: time + EXPIRING,
            iat: time,
            is_super: Some(user.is_super),
        }
    }
}

impl From<&Car> for Claims {
    fn from(car: &Car) -> Claims {
        let time = Utc::now().timestamp();
        Claims {
            token_type: TokenType::Car,
            sub: car.id,
            exp: time + EXPIRING,
            email: Option::None,
            iat: time,
            is_super: Option::None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserAuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CarAuthPayload {
    pub id: Uuid,
    pub key: String,
}
