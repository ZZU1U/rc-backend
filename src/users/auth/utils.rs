use jsonwebtoken::{encode, Header};
use anyhow::Result;
use crate::users::models::User;
use crate::users::auth::{vars::KEYS, models::Claims};

pub async fn generate_token(user: &User) -> Result<String> {
    let claims = Claims::new(user);

    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        Ok(
            encode(&Header::default(), &claims, &KEYS.encoding)?
        )
    })
    .await
    .unwrap()?)
}
