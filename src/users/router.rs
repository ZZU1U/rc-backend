use axum::{
    routing::{get, post, MethodRouter},
    Router,
};

//use crate::users::auth::login;

pub fn register_route() -> Router {
    async fn handler() -> &'static str {
        "Let's register you, my boy"
    }

    Router::new()
        .route("/register", get(handler))
        //.route("/login", post(login))
}
