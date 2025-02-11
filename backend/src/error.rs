use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };
        
        match self {
            AppError::ConfigError(_) => {
                HttpResponse::InternalServerError().json(error_response)
            }
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(error_response)
            }
            AppError::ValidationError(_) => {
                HttpResponse::BadRequest().json(error_response)
            }
            AppError::AuthError(_) => {
                HttpResponse::Unauthorized().json(error_response)
            }
            AppError::NotFound(_) => {
                HttpResponse::NotFound().json(error_response)
            }
            AppError::Conflict(_) => {
                HttpResponse::Conflict().json(error_response)
            }
        }
    }
} 