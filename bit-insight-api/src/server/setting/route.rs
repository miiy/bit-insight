use super::handler;
use actix_web::web;

// GET /setting
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/settings")
            .service(web::resource("").route(web::put().to(handler::update))),
    );
}
