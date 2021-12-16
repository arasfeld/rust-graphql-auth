#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

mod auth;
mod database;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Database
    let pool = database::get_db_pool();

    // Server
    let server = HttpServer::new(move || App::new().data(pool.clone()).configure(auth::routes))
        .bind("0.0.0.0:3000")
        .unwrap()
        .run();

    // Server running
    println!("🚀 Server ready at http://localhost:3000");

    // Awaiting server to exit
    server.await
}
