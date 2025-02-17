mod auth;
mod config;
mod db;
mod error;
mod models;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use serde_json::json;
use tracing::info;

use crate::{
    auth::routes::auth_routes,
    config::Config,
    db::Database,
    error::{AppError, ErrorResponse},
};

async fn index() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "name": "Gamie API",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "auth": {
                "signup": "POST /auth/signup",
                "login": "POST /auth/login",
                "user": "GET /auth/user/{id}"
            }
        }
    }))
}

async fn not_found() -> Result<HttpResponse, AppError> {
    Err(AppError::NotFound(ErrorResponse {
        code: "NOT_FOUND".to_string(),
        message: "The requested resource was not found".to_string(),
        field: None,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env().expect("Server configuration");

    // Initialize database
    let db = Database::connect(&config.database_url)
        .await
        .expect("Database connection failed");

    // Configure rate limiting
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2) // Allow 2 requests per second
        .burst_size(5) // Allow bursts of up to 5 requests
        .finish()
        .unwrap();

    let server_url = format!("http://{}:{}", config.host, config.port);
    info!("Starting server at {}", server_url);

    let config_clone = config.clone();
    HttpServer::new(move || {
        // Choose allowed origins based on environment
        let env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
        let cors = if env == "development" {
            Cors::default()
                .allowed_origin("http://localhost:5173")
                .allowed_origin("http://127.0.0.1:5173")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
                .expose_headers(vec![CONTENT_TYPE, AUTHORIZATION])
                .supports_credentials()
                .max_age(3600)
        } else {
            // Production: use your production frontend URL from your config
            Cors::default()
                .allowed_origin(&config_clone.frontend_url)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
                .supports_credentials()
                .max_age(3600)
        };

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(config_clone.clone()))
            .service(web::resource("/").to(index))
            .configure(auth_routes)
            .default_service(web::route().to(not_found))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
