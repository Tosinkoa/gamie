use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
}

#[derive(Debug)]
pub enum AppError {
    ValidationError(ErrorResponse),
    DatabaseError(ErrorResponse),
    AuthError(ErrorResponse),
    Conflict(ErrorResponse),
    NotFound(ErrorResponse),
    ConfigError(ErrorResponse),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ValidationError(err) => write!(f, "Validation error: {}", err.message),
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err.message),
            AppError::AuthError(err) => write!(f, "Authentication error: {}", err.message),
            AppError::Conflict(err) => write!(f, "Conflict error: {}", err.message),
            AppError::NotFound(err) => write!(f, "Not found: {}", err.message),
            AppError::ConfigError(err) => write!(f, "Configuration error: {}", err.message),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ValidationError(err) => HttpResponse::BadRequest().json(err),
            AppError::DatabaseError(err) => HttpResponse::InternalServerError().json(err),
            AppError::AuthError(err) => HttpResponse::Unauthorized().json(err),
            AppError::Conflict(err) => HttpResponse::Conflict().json(err),
            AppError::NotFound(err) => HttpResponse::NotFound().json(err),
            AppError::ConfigError(err) => HttpResponse::InternalServerError().json(err),
        }
    }
}
