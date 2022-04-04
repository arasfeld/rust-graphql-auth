use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::{get, post}, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use rust_graphql_auth::{
    database,
    graphql::{AppSchema, MutationRoot, QueryRoot},
    handlers
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .pretty()
        .init();

    let pg_pool = database::get_db_pool().await;

    let schema: AppSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pg_pool.to_owned())
        .finish();

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(schema))
        .layer(Extension(pg_pool))
        .into_inner();

    let app = Router::new()
        .route("/register", post(handlers::register))
        .route("/sign_in", post(handlers::sign_in))
        .route(
            "/graphql",
            get(handlers::graphql_playground).post(handlers::graphql),
        )
        .layer(middleware_stack);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server ready at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
