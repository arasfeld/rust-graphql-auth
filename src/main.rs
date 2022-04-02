use axum::{extract::Extension, routing::post, Router};
use std::net::SocketAddr;

use rust_graphql_auth::{
    database,
    handlers::{log_in, register}
};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Database
    let pool = database::get_db_pool().await;

    // App
    let app = Router::new()
        .route("/login", post(log_in))
        .route("/register", post(register))
        .layer(Extension(pool));

    // Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server ready at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
