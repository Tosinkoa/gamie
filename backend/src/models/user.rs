use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]  // Never send password hash to client
    pub password_hash: String,
    pub email_verified: bool,
    pub failed_login_attempts: i32,
    pub last_login_attempt: Option<DateTime<Utc>>,
    pub account_locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 30, message = "username must be between 3 and 30 characters"))]
    #[validate(regex(path = "USERNAME_REGEX", message = "username must contain only letters, numbers, and underscores"))]
    pub username: String,
    
    #[validate(email(message = "invalid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "password must be at least 8 characters long"))]
    #[validate(regex(path = "PASSWORD_REGEX", message = "password must contain at least one uppercase letter, one lowercase letter, one number, and one special character"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "passwords do not match"))]
    pub password_verify: String,
}

lazy_static::lazy_static! {
    static ref USERNAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    static ref PASSWORD_REGEX: regex::Regex = regex::Regex::new(
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"
    ).unwrap();
    static ref COMMON_PASSWORDS: std::collections::HashSet<&'static str> = {
        let mut set = std::collections::HashSet::new();
        set.insert("Password123!");
        set.insert("Admin123!");
        // Add more common passwords as needed
        set
    };
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            username,  // We'll validate and sanitize before this point
            email: email.to_lowercase(),    // Normalize email
            password_hash,
            email_verified: false,
            failed_login_attempts: 0,
            last_login_attempt: None,
            account_locked_until: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_account_locked(&self) -> bool {
        if let Some(locked_until) = self.account_locked_until {
            return Utc::now() < locked_until;
        }
        false
    }
}

impl CreateUserDto {
    pub fn is_password_common(&self) -> bool {
        COMMON_PASSWORDS.contains(self.password.as_str())
    }
    
    pub fn sanitize(&mut self) {
        // Instead of using sanitize_html directly, we'll just remove any non-alphanumeric characters
        self.username = self.username.chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        self.email = self.email.to_lowercase();
    }
} 