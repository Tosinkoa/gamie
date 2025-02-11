use std::env;
use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Config, AppError> {
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .map_err(|_| AppError::ConfigError("Invalid PORT".to_string()))?,
            database_url: env::var("DATABASE_URL").map_err(|_| {
                AppError::ConfigError("DATABASE_URL must be set".to_string())
            })?,
            jwt_secret: env::var("JWT_SECRET").map_err(|_| {
                AppError::ConfigError("JWT_SECRET must be set".to_string())
            })?,
        })
    }
} 