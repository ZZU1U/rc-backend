use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, RequestPartsExt, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde_json::json;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use crate::users::models::User;
use crate::cars::models::Car;
use crate::app_state::{KEYS, EXPIRING};



#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
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

#[derive(Debug, Serialize)]
pub struct JWT {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum TokenType {
    #[serde(rename = "Car")]
    Car,
    #[serde(rename = "User")]
    User
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    token_type: TokenType,
    username: String,
    exp: DateTime<Utc>,
    is_super: Option<bool>,
    iat: DateTime<Utc>
}

impl Claims {
    fn new<T>(args: T) -> Claims
        where T: Into<Claims>
    {
        args.into()
    }
}

impl From<&User> for Claims {
    fn from(user: &User) -> Claims {
        let time = Utc::now();
        Claims {
            token_type: TokenType::User,
            sub: user.id,
            username: user.username.clone(),
            exp: time + EXPIRING,
            iat: time,
            is_super: Some(user.is_super)
        }
    }
}

impl From<&Car> for Claims {
    fn from(car: &Car) -> Claims {
        let time = Utc::now();
        Claims {
            token_type: TokenType::Car,
            sub: car.id,
            username: car.name.clone(),
            exp: time + EXPIRING,
            iat: time,
            is_super: Option::None
        }
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}


pub fn create_token(user: &User) -> String {
    let claims = Claims::new(user);

    let token = encode(&Header::default(), &claims, &KEYS.encoding).unwrap();

    return token;
}
