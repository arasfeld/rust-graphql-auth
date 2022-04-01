use axum::{routing::post, Router};

mod errors;
mod handlers;
mod model;
mod service;
mod utils;

pub fn router() -> Router {
    Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register))
}
