use crate::auth::{
    handlers::{get_current_user, get_user_by_id, login, refresh_token, signup},
    middleware::AuthMiddleware,
};
use actix_web::web;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/signup").route(web::post().to(signup)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/refresh").route(web::post().to(refresh_token)))
            .service(
                web::scope("")
                    .wrap(AuthMiddleware::new(
                        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                    ))
                    .service(web::resource("/user/{id}").route(web::get().to(get_user_by_id)))
                    .service(web::resource("/me").route(web::get().to(get_current_user))),
            ),
    );
}
