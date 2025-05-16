use super::passwords::hash_password;
use crate::app_state::AppState;
use crate::users::models::User;
use std::env;

pub async fn check_default_admin(state: AppState) {
    let admin_email = env::var("DEFAULT_ADMIN_EMAIL").expect("Admin email is not specified");
    let admin_pwd = env::var("DEFAULT_ADMIN_PASSWORD").expect("Admin password is not specified");

    let existing_admin = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM "user"
        WHERE (email = $1)
        "#,
        admin_email
    )
    .fetch_one(&state.pool)
    .await;

    match existing_admin {
        Ok(admin) => {
            println!("Existing admin {:?}", admin);
            return;
        }
        Err(_) => {
            println!("No admin or something went wrong");
        }
    };

    let admin_hash = hash_password(admin_pwd).await.unwrap();

    let admin = sqlx::query_as!(
        User,
        r#"
        INSERT INTO "user" (id, email, password_hash, is_verified, is_super)
        VALUES ($1, $2, $3, true, true)
        RETURNING *
        "#,
        uuid::uuid!("019688b5-075d-73d1-99fe-1708ac57bdd8"),
        admin_email,
        admin_hash
    )
    .fetch_one(&state.pool)
    .await;

    println!("New default admin is {:?}", admin);
}
