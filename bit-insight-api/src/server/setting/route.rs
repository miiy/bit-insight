use super::handler;
use actix_web::web;

// GET /setting/{key}
// PUT /setting/{key}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/settings").service(
            web::resource("/{key}")
                .route(web::get().to(handler::detail))
                .route(web::put().to(handler::update)),
        ),
    );
}
