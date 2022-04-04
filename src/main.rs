use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::{get, post}, Router};
use std::net::SocketAddr;

use rust_graphql_auth::{
    database,
    graphql::{AppSchema, MutationRoot, QueryRoot},
    handlers
};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Database
    let pg_pool = database::get_db_pool().await;

    // GraphQL
    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pg_pool.to_owned())
        .finish();

    // App
    let app = Router::new()
        .route("/register", post(handlers::register))
        .route("/sign_in", post(handlers::sign_in))
        .route(
            "/graphql",
            get(handlers::graphql_playground).post(handlers::graphql),
        )
        .layer(Extension(pg_pool))
        .layer(Extension(schema));

    // Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server ready at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
