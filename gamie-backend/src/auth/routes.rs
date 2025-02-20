use crate::auth::handlers::{login, logout, refresh_token, signup};
use actix_web::web;
use tracing::info;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuring auth routes");
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/signup").route(web::post().to(signup)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/refresh").route(web::post().to(refresh_token)))
            .service(web::resource("/logout").route(web::post().to(logout))),
    );
    info!("Auth routes configured");
}
