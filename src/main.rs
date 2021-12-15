#[macro_use]
extern crate diesel;

use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};

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

    // Server
    let app = Route::new()
        .at("/auth/basic", auth::basic::basic_auth)
        .with(AddData::new(pool));
    let server = Server::new(TcpListener::bind(addr))
        .name("rust-graphql-auth")
        .run(app);

    // Server running
    println!("🚀 Server ready at http://localhost:{}", port);

    // Awaiting server to exit
    server.await
}
