use poem::{
  get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server,
};

#[handler]
fn hello(Path(name): Path<String>) -> String {
  format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  // Load environment variables from .env file
  dotenv::dotenv().ok();

  // Server address
  let port = dotenv::var("PORT").unwrap();
  let addr = format!("127.0.0.1:{}", port);

  // Server
  let app = Route::new().at("/hello/:name", get(hello)).with(Tracing);
  let server = Server::new(TcpListener::bind(addr))
    .name("rust-graphql-auth")
    .run(app);

  // Server running
  println!("🚀 Server ready at http://localhost:{}", port);

  // Awaiting server to exit
  server.await
}
