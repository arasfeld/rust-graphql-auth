use actix_web::web;

mod basic;
mod utils;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/basic").route(web::post().to(basic::basic_auth))),
    );
}
