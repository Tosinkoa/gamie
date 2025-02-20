use super::handlers::{get_current_user, get_user_by_username};
use crate::auth::middleware::AuthMiddleware;
use actix_web::web;
use tracing::info;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuring user routes");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    info!("JWT_SECRET loaded");
    let auth = AuthMiddleware::new(jwt_secret);

    cfg.service(
        web::scope("/users")
            // Protected routes (must come first)
            .service(
                web::resource("/me")
                    .wrap(auth)
                    .route(web::get().to(get_current_user)),
            )
            // Public routes
            .service(web::resource("/{username}").route(web::get().to(get_user_by_username))),
    );
    info!("User routes configured");
}
