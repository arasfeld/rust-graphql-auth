use axum::{extract::Extension, Router};
use std::net::SocketAddr;

use rust_graphql_auth::{auth, database};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Database
    let pool = database::get_db_pool().await;

    // App
    let app = Router::new()
        .nest("/auth", auth::router())
        .layer(Extension(pool));

    // Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server ready at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
