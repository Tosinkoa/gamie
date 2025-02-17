use actix_web::web;
use super::handlers::{signup, get_user_by_id, login};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/users/{id}", web::get().to(get_user_by_id))
    );
} 