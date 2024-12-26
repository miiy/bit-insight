use super::auth;
use super::health;
use super::index;
use super::article;
use super::material;
use super::profile;
use super::setting;
use super::user;
use crate::middleware::authentication;
use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    let auth_middleware = authentication::Authentication;

    // public
    index::route::init_routes(cfg);
    health::route::init_routes(cfg);
    auth::route::init_routes(cfg);
    user::route::init_routes(cfg);
    article::route::init_routes(cfg);
    material::route::init_routes(cfg);

    // private
    cfg.service(
        web::scope("")
            .wrap(auth_middleware)
            .configure(profile::route::init_routes)
            .configure(setting::route::init_routes),
    );
}
