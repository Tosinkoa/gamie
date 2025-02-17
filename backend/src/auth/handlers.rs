use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};
use validator::Validate;

use crate::{
    db::Database,
    error::{AppError, ErrorResponse},
    models::user::{CreateUserDto, User},
};

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub username_or_email: String,
    pub password: String,
}

pub async fn signup(
    db: web::Data<Database>,
    mut payload: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    // Sanitize input and convert to lowercase
    payload.username = payload.username.to_lowercase();
    payload.email = payload.email.to_lowercase();
    payload.sanitize();

    // Validate the request payload
    payload.validate().map_err(|e| {
        error!("Validation error during signup: {}", e);
        AppError::ValidationError(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: e.to_string(),
            field: None,
        })
    })?;

    // Check for common passwords
    if payload.is_password_common() {
        return Err(AppError::ValidationError(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "This password is too common. Please choose a stronger password.".to_string(),
            field: Some("password".to_string()),
        }));
    }

    let users_collection = db.get_users_collection();

    // Check both username and email existence in a single query (case insensitive)
    let existing_user = users_collection
        .find_one(
            doc! {
                "$or": [
                    {
                        "username": payload.username.to_lowercase()
                    },
                    {
                        "email": payload.email.to_lowercase()
                    }
                ]
            },
            None,
        )
        .await
        .map_err(|e| {
            error!("Database error checking user existence: {}", e);
            AppError::DatabaseError(ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: "Database error occurred".to_string(),
                field: None,
            })
        })?;

    // Check which field caused the conflict
    if let Some(existing) = existing_user {
        if existing.email == payload.email {
            // Both are already lowercase
            return Err(AppError::Conflict(ErrorResponse {
                code: "EMAIL_EXISTS".to_string(),
                message: "Account already exists".to_string(),
                field: Some("email".to_string()),
            }));
        } else {
            return Err(AppError::Conflict(ErrorResponse {
                code: "USERNAME_TAKEN".to_string(),
                message: "Username is already taken".to_string(),
                field: Some("username".to_string()),
            }));
        }
    }

    // Hash the password with Argon2id
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Password hashing error: {}", e);
            AppError::AuthError(ErrorResponse {
                code: "ACCOUNT_CREATION_ERROR".to_string(),
                message: "Error creating account".to_string(),
                field: Some("password".to_string()),
            })
        })?
        .to_string();

    info!("Generated password hash during signup: {}", password_hash);

    // Create new user
    let user = User::new(
        payload.username.clone(),
        payload.email.clone(),
        password_hash,
    );

    info!("User object before saving: {:?}", user);

    // Insert user into database
    let insert_result = users_collection
        .insert_one(&user, None)
        .await
        .map_err(|e| {
            error!("Database error during user creation: {}", e);
            error!("Attempted to insert user: {:?}", user);
            AppError::DatabaseError(ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: format!("Error creating user account: {}", e),
                field: None,
            })
        })?;

    // Verify the user was saved correctly
    if let Some(id) = insert_result.inserted_id.as_object_id() {
        if let Ok(Some(saved_user)) = users_collection.find_one(doc! { "_id": id }, None).await {
            info!(
                "Saved user verification - Password hash exists: {}, length: {}",
                !saved_user.password_hash.is_empty(),
                saved_user.password_hash.len()
            );
        }
    }

    info!("New user created: {}", user.username);

    Ok(HttpResponse::Created().json(json!({
        "message": "User created successfully",
        "user": {
            "username": user.username,
            "email": user.email
        }
    })))
}

pub async fn get_user_by_id(
    db: web::Data<Database>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let object_id = mongodb::bson::oid::ObjectId::parse_str(&*user_id).map_err(|e| {
        error!("Invalid user ID format: {}", e);
        AppError::ValidationError(ErrorResponse {
            code: "INVALID_USER_ID".to_string(),
            message: "Invalid user ID format".to_string(),
            field: Some("user_id".to_string()),
        })
    })?;

    let users_collection = db.get_users_collection();

    let user = users_collection
        .find_one(doc! { "_id": object_id }, None)
        .await
        .map_err(|e| {
            error!("Database error fetching user: {}", e);
            AppError::DatabaseError(ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: "Error fetching user details".to_string(),
                field: None,
            })
        })?
        .ok_or_else(|| {
            error!("User not found with ID: {}", user_id);
            AppError::NotFound(ErrorResponse {
                code: "USER_NOT_FOUND".to_string(),
                message: "User not found".to_string(),
                field: Some("user_id".to_string()),
            })
        })?;

    println!("==========================================");
    println!("Data: {}", user.username);
    println!("==========================================");

    info!("User found: {}", user.username);

    Ok(HttpResponse::Ok().json(json!({
        "user": {
            "id": user.id_to_string(),
            "username": user.username,
            "email": user.email,
            "email_verified": user.email_verified,
            "created_at": user.created_at,
            "updated_at": user.updated_at
        }
    })))
}

pub async fn login(
    db: web::Data<Database>,
    payload: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    let users_collection = db.get_users_collection();

    // Trim whitespace and convert to lowercase
    let username_or_email = payload.username_or_email.trim().to_lowercase();
    let password = payload.password.trim();

    if username_or_email.is_empty() {
        return Err(AppError::ValidationError(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Username or email is required".to_string(),
            field: Some("username_or_email".to_string()),
        }));
    }

    if password.is_empty() {
        return Err(AppError::ValidationError(ErrorResponse {
            code: "VALIDATION_ERROR".to_string(),
            message: "Password is required".to_string(),
            field: Some("password".to_string()),
        }));
    }

    // Find user by username or email (case insensitive)
    let user = users_collection
        .find_one(
            doc! {
                "$or": [
                    { "username": &username_or_email },
                    { "email": &username_or_email }
                ]
            },
            None,
        )
        .await
        .map_err(|e| {
            error!("Database error during login: {}", e);
            AppError::DatabaseError(ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: e.to_string(),
                field: None,
            })
        })?;

    info!("Login attempt - Found user: {}", username_or_email);

    if let Some(ref user) = user {
        info!(
            "User details - Username: {}, Email: {}, Has password: {}",
            user.username,
            user.email,
            !user.password_hash.is_empty()
        );
    }

    let mut user = user.ok_or_else(|| {
        error!("No user found with username/email: {}", username_or_email);
        AppError::AuthError(ErrorResponse {
            code: "INVALID_CREDENTIALS".to_string(),
            message: "Invalid username/email or password".to_string(),
            field: None,
        })
    })?;

    // Check if account is locked
    if user.is_account_locked() {
        let wait_time = user.get_lockout_wait_time();
        return Err(AppError::AuthError(ErrorResponse {
            code: "ACCOUNT_LOCKED".to_string(),
            message: format!(
                "Account is temporarily locked. Please try again in {} minutes.",
                wait_time
            ),
            field: None,
        }));
    }

    // Verify password
    if !user.verify_password(password) {
        // Record failed login attempt
        user.record_failed_login();

        // Update user in database with new failed login count
        users_collection
            .replace_one(doc! { "_id": user.id }, &user, None)
            .await
            .map_err(|e| {
                error!("Database error updating failed login attempts: {}", e);
                AppError::DatabaseError(ErrorResponse {
                    code: "DATABASE_ERROR".to_string(),
                    message: e.to_string(),
                    field: None,
                })
            })?;

        return Err(AppError::AuthError(ErrorResponse {
            code: "INVALID_CREDENTIALS".to_string(),
            message: "Invalid username/email or password".to_string(),
            field: None,
        }));
    }

    // Reset login attempts on successful login
    user.reset_login_attempts();

    // Update user in database
    users_collection
        .replace_one(doc! { "_id": user.id }, &user, None)
        .await
        .map_err(|e| {
            error!("Database error updating user after successful login: {}", e);
            AppError::DatabaseError(ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: e.to_string(),
                field: None,
            })
        })?;

    info!("User logged in successfully: {}", user.username);

    Ok(HttpResponse::Ok().json(json!({
        "message": "Login successful",
        "user": {
            "username": user.username,
            "email": user.email,
        }
    })))
}
