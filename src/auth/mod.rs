use actix_web::web;

mod handlers;
mod utils;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth").service(web::resource("/login").route(web::post().to(handlers::login))),
    );
}
