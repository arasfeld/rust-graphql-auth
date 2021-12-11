use poem::{
  get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt, Route, Server,
};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let app = Route::new().at("/hello/:name", get(hello)).with(Tracing);
  let server = Server::new(TcpListener::bind("127.0.0.1:3000"))
    .name("rust-graphql-auth")
    .run(app);
  println!("🚀 Server ready at http://localhost:3000");
  server.await
}
