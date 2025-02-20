mod auth;
mod config;
mod db;
mod error;
mod game;
mod models;
mod user;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use serde_json::json;
use tracing::info;
use std::env;

use crate::{
    auth::routes::auth_routes,
    config::Config,
    db::Database,
    error::{AppError, ErrorResponse},
    user::routes::user_routes,
    game::game_routes,
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
            },
            "game": {
                "websocket": "WS /game/ws",
                "create_room": "POST /game/rooms",
                "get_room": "GET /game/rooms/{room_id}"
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

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{}:{}", host, port);
    info!("Starting server at http://{}", server_url);

    let config_clone = config.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(config_clone.clone()))
            .service(web::resource("/").to(index))
            .configure(auth_routes)
            .configure(user_routes)
            .configure(game_routes)
            .default_service(web::route().to(not_found))
    })
    .bind(server_url)?
    .run()
    .await
}
