use axum::{
    routing::{get, post, MethodRouter},
    Router,
};

pub fn cars_router() -> Router {
    Router::new()
        .route("/cars", get(get_cars))
        .route("/cars", post(create_car))
}

async fn get_cars() -> &'static str {
    "There are all the cars:"
}

async fn create_car() -> &'static str {
    "Creating new cat"
}
