#[macro_use]
extern crate diesel;

use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

mod auth;
mod database;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Server address
    let port = dotenv::var("PORT").unwrap();
    let addr = format!("127.0.0.1:{}", port);

    // Database
    let pool = database::get_db_pool();

    // Auth service
    let auth_service_addr = format!("http://localhost:{}/auth", port);
    let auth_service =
        OpenApiService::new(auth::AuthApi, "Auth Service", "1.0").server(auth_service_addr);
    let auth_ui = auth_service.swagger_ui();

    // Server
    let app = Route::new()
        .nest("/auth", auth_service)
        .nest("/", auth_ui)
        .with(AddData::new(pool));
    let server = Server::new(TcpListener::bind(addr))
        .name("rust-graphql-auth")
        .run(app);

    // Server running
    println!("🚀 Server ready at http://localhost:{}", port);

    // Awaiting server to exit
    server.await
}
