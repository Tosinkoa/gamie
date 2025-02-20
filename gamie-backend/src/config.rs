use crate::error::{AppError, ErrorResponse};
use serde::Deserialize;

/// Server configuration loaded from environment variables
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Host address to bind to (e.g., "0.0.0.0")
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// MongoDB connection URL
    pub database_url: String,
    /// Frontend URL for CORS
    pub frontend_url: String,
    /// JWT secret for authentication
    #[allow(dead_code)]
    pub jwt_secret: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Config, AppError> {
        dotenv::dotenv().ok();

        let host = std::env::var("HOST").map_err(|_| {
            AppError::ConfigError(ErrorResponse {
                code: "MISSING_CONFIG".to_string(),
                message: "HOST must be set".to_string(),
                field: Some("host".to_string()),
            })
        })?;

        let port = std::env::var("PORT")
            .map_err(|_| {
                AppError::ConfigError(ErrorResponse {
                    code: "MISSING_CONFIG".to_string(),
                    message: "PORT must be set".to_string(),
                    field: Some("port".to_string()),
                })
            })?
            .parse()
            .map_err(|_| {
                AppError::ConfigError(ErrorResponse {
                    code: "INVALID_CONFIG".to_string(),
                    message: "PORT must be a valid number".to_string(),
                    field: Some("port".to_string()),
                })
            })?;

        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            AppError::ConfigError(ErrorResponse {
                code: "MISSING_CONFIG".to_string(),
                message: "DATABASE_URL must be set".to_string(),
                field: Some("database_url".to_string()),
            })
        })?;

        let frontend_url = std::env::var("FRONTEND_URL").map_err(|_| {
            AppError::ConfigError(ErrorResponse {
                code: "MISSING_CONFIG".to_string(),
                message: "FRONTEND_URL must be set".to_string(),
                field: Some("frontend_url".to_string()),
            })
        })?;

        let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| {
            AppError::ConfigError(ErrorResponse {
                code: "MISSING_CONFIG".to_string(),
                message: "JWT_SECRET must be set".to_string(),
                field: Some("jwt_secret".to_string()),
            })
        })?;

        Ok(Config {
            host,
            port,
            database_url,
            frontend_url,
            jwt_secret,
        })
    }
}
