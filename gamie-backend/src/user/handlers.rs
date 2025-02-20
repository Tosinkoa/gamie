use actix_web::{web, HttpResponse};
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::json;
use tracing::{error, info};

use crate::{
    db::Database,
    error::{AppError, ErrorResponse},
};

pub async fn get_user_by_username(
    db: web::Data<Database>,
    username: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("get_user_by_username called with username: {}", username);
    let users_collection = db.get_users_collection();

    let user = users_collection
        .find_one(doc! { "username": username.to_lowercase() }, None)
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
            error!("User not found with username: {}", username);
            AppError::NotFound(ErrorResponse {
                code: "USER_NOT_FOUND".to_string(),
                message: "User not found".to_string(),
                field: Some("username".to_string()),
            })
        })?;

    info!("Found user by username: {}", user.username);
    Ok(HttpResponse::Ok().json(json!({
        "user": {
            "username": user.username,
            "profile_picture": user.profile_picture
        }
    })))
}

pub async fn get_current_user(
    db: web::Data<Database>,
    user_id: web::ReqData<ObjectId>,
) -> Result<HttpResponse, AppError> {
    info!("get_current_user called with user_id: {:?}", user_id);
    let users_collection = db.get_users_collection();

    let user = users_collection
        .find_one(doc! { "_id": *user_id }, None)
        .await
        .map_err(|e| {
            error!("Database error fetching current user: {}", e);
            AppError::DatabaseError(ErrorResponse {
                code: "DATABASE_ERROR".to_string(),
                message: "Error fetching user details".to_string(),
                field: None,
            })
        })?
        .ok_or_else(|| {
            error!("User not found with ID: {:?}", user_id);
            AppError::NotFound(ErrorResponse {
                code: "USER_NOT_FOUND".to_string(),
                message: "User not found".to_string(),
                field: None,
            })
        })?;

    info!("Found current user: {:?}", user);
    Ok(HttpResponse::Ok().json(json!({
        "user": {
            "id": user.id_to_string(),
            "username": user.username,
            "email": user.email,
            "profile_picture": user.profile_picture,
            "email_verified": user.email_verified,
            "created_at": user.created_at,
            "updated_at": user.updated_at
        }
    })))
}
