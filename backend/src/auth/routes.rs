use super::handlers::{get_user_by_id, login, signup};
use actix_web::web;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/users/{id}", web::get().to(get_user_by_id)),
    );
}
