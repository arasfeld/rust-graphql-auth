#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};

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
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(auth::routes)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run();

    // Server running
    println!("🚀 Server ready at http://localhost:3000");

    // Awaiting server to exit
    server.await
}
