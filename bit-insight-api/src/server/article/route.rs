use super::handler;
use actix_web::web;

// GET /articles
// GET /articles/{id}
// POST /articles
// PUT /articles/{id}
// DELETE /articles/{id}
// POST /articles/push
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/articles")
            .service(
                web::resource("")
                    .route(web::get().to(handler::list))
                    .route(web::post().to(handler::create)),
            )
            .service(web::resource("/push").route(web::post().to(handler::push)))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(handler::detail))
                    .route(web::put().to(handler::update))
                    .route(web::delete().to(handler::delete)),
            ),
    );
}
