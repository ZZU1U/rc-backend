use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use anyhow::Result;

pub async fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
    Ok(tokio::task::spawn_blocking(move || -> Result<String, argon2::password_hash::Error> {
        let argon = Argon2::default();

        let salt = SaltString::generate(OsRng);
        Ok(
            argon.hash_password(password.as_bytes(), &salt)?.to_string(),
        )
    })
    .await
    .unwrap()?)
}

pub async fn check_password(hash: String, password: String) -> Result<bool, argon2::password_hash::Error> {
    Ok(tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash)?;

        Ok::<bool, argon2::password_hash::Error>(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    })
    .await.unwrap()?)
}
