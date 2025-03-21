use std::time::{Duration, SystemTime};
use axum::{
    response::IntoResponse, 
    http::StatusCode,
    Json,
};
use axum_extra::{
    headers::{
        authorization::{Basic, Bearer},
        Authorization,
    },
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

const SECRET_SIGNING_KEY: &[u8] = b"keep_th1s_@_secret";

#[derive(Serialize, Deserialize)]
pub struct OurJwtPayload {
    pub sub: String,
    pub exp: usize,
}

impl OurJwtPayload {
    pub fn new(sub: String) -> Self {
        // expires by default in 60 minutes from now
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(60 * 60))
            .expect("valid timestamp")
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("valid duration")
            .as_secs() as usize;

        OurJwtPayload { sub, exp }
    }
}

/// login endpoint
pub async fn login(
    TypedHeader(Authorization(creds)): TypedHeader<Authorization<Basic>>,
) -> impl IntoResponse {
    if creds.username() != "admin" || creds.password() != "password" {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized"})),
        );
    };

    let Ok(jwt) = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &OurJwtPayload::new(creds.username().to_string()),
        &jsonwebtoken::EncodingKey::from_secret(SECRET_SIGNING_KEY),
    ) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to generate token"})),
        );
    };

    (StatusCode::OK, Json(json!({"jwt": jwt})))
}
