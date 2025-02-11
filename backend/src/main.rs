mod auth;
mod config;
mod db;
mod error;
mod models;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_governor::{Governor, GovernorConfigBuilder};
use dotenv::dotenv;
use tracing::info;
use std::time::Duration;

use crate::{
    auth::routes::auth_routes,
    config::Config,
    db::Database,
};

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
    
    info!("Starting server at http://{}:{}", config.host, config.port);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // TODO: Configure CORS properly for production
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
            .configure(auth_routes)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
} 