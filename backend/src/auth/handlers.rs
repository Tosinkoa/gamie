use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use mongodb::bson::doc;
use validator::Validate;
use tracing::{error, info};
use regex;
use serde_json::json;

use crate::{
    db::Database,
    error::AppError,
    models::user::{CreateUserDto, User},
};

pub async fn signup(
    db: web::Data<Database>,
    mut payload: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    // Sanitize input
    payload.sanitize();
    
    // Validate the request payload (this will also check if passwords match due to the must_match validation)
    payload.validate().map_err(|e| {
        error!("Validation error during signup: {}", e);
        AppError::ValidationError(e.to_string())
    })?;
    
    // Check for common passwords
    if payload.is_password_common() {
        return Err(AppError::ValidationError(
            "This password is too common. Please choose a stronger password.".to_string(),
        ));
    }
    
    // Additional explicit password match check (although validator already checks this)
    if payload.password != payload.password_verify {
        return Err(AppError::ValidationError(
            "Passwords do not match".to_string(),
        ));
    }
    
    let users_collection = db.get_users_collection();
    
    // Check if username already exists (case insensitive)
    if users_collection
        .find_one(
            doc! {
                "username": {
                    "$regex": format!("^{}$", regex::escape(&payload.username)),
                    "$options": "i"
                }
            },
            None,
        )
        .await
        .map_err(AppError::DatabaseError)?
        .is_some()
    {
        return Err(AppError::Conflict("Username already taken".to_string()));
    }
    
    // Check if email already exists (case insensitive)
    if users_collection
        .find_one(
            doc! {
                "email": payload.email.to_lowercase()
            },
            None,
        )
        .await
        .map_err(AppError::DatabaseError)?
        .is_some()
    {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }
    
    // Hash the password with Argon2id (memory-hard algorithm)
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Password hashing error: {}", e);
            AppError::AuthError("Error creating account".to_string())
        })?
        .to_string();
    
    // Create new user
    let user = User::new(
        payload.username.clone(),
        payload.email.clone(),
        password_hash,
    );
    
    // Insert user into database
    users_collection
        .insert_one(&user, None)
        .await
        .map_err(|e| {
            error!("Database error during user creation: {}", e);
            AppError::DatabaseError(e)
        })?;
    
    info!("New user created: {}", user.username);
    
    // Return success but don't include sensitive data
    Ok(HttpResponse::Created().json(json!({
        "message": "User created successfully",
        "username": user.username,
        "email": user.email,
    })))
} 