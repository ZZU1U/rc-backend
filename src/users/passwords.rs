use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use password_hash::rand_core::OsRng;


pub async fn hash_password(password: String) -> Result<String, &'static str> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String, &'static str> {
        let salt = SaltString::generate(OsRng);
        Ok(
            PasswordHash::generate(Argon2::default(), password, &salt).unwrap().to_string(),
        )
    })
    .await
    .unwrap()?)
}

