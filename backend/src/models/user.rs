use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing, default)]
    // Never send password hash to client and allow missing during checks
    password_hash: String,
    #[serde(default)]
    pub email_verified: bool,
    #[serde(default)]
    pub failed_login_attempts: i32,
    #[serde(default)]
    pub last_login_attempt: Option<DateTime<Utc>>,
    #[serde(default)]
    pub account_locked_until: Option<DateTime<Utc>>,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_verify: String,
}

lazy_static::lazy_static! {
    static ref USERNAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    static ref PASSWORD_REGEX: regex::Regex = regex::Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").unwrap();
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
            username,
            email: email.to_lowercase(),
            password_hash,
            email_verified: false,
            failed_login_attempts: 0,
            last_login_attempt: None,
            account_locked_until: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        use argon2::PasswordHash;
        use argon2::PasswordVerifier;

        if let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) {
            argon2::Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }

    pub fn is_account_locked(&self) -> bool {
        if let Some(locked_until) = self.account_locked_until {
            Utc::now() < locked_until
        } else {
            false
        }
    }

    pub fn record_failed_login(&mut self) {
        self.failed_login_attempts += 1;
        self.last_login_attempt = Some(Utc::now());

        // Lock account after 5 failed attempts
        if self.failed_login_attempts >= 5 {
            self.account_locked_until = Some(Utc::now() + chrono::Duration::minutes(30));
        }
    }

    pub fn reset_login_attempts(&mut self) {
        self.failed_login_attempts = 0;
        self.account_locked_until = None;
        self.last_login_attempt = Some(Utc::now());
    }

    pub fn get_lockout_wait_time(&self) -> i64 {
        if let Some(locked_until) = self.account_locked_until {
            let now = Utc::now();
            let duration = locked_until.signed_duration_since(now);
            (duration.num_seconds() as f64 / 60.0).ceil() as i64
        } else {
            0
        }
    }
}

impl Validate for CreateUserDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Validate username
        if self.username.len() < 3 || self.username.len() > 30 {
            errors.add(
                "username",
                ValidationError::new("username must be between 3 and 30 characters"),
            );
        }
        if !USERNAME_REGEX.is_match(&self.username) {
            errors.add(
                "username",
                ValidationError::new(
                    "username must contain only letters, numbers, and underscores",
                ),
            );
        }

        // Validate email
        if !self.email.contains('@') {
            errors.add("email", ValidationError::new("invalid email address"));
        }

        // Validate password
        if self.password.len() < 8 {
            errors.add(
                "password",
                ValidationError::new("password must be at least 8 characters long"),
            );
        }
        if !PASSWORD_REGEX.is_match(&self.password) {
            errors.add("password", ValidationError::new("invalid password format"));
        }
        if self.is_password_common() {
            errors.add("password", ValidationError::new("password is too common"));
        }

        // Validate password strength
        let has_lowercase = self.password.chars().any(|c| c.is_ascii_lowercase());
        let has_uppercase = self.password.chars().any(|c| c.is_ascii_uppercase());
        let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
        let has_special = self.password.chars().any(|c| "@$!%*?&".contains(c));

        if !has_lowercase || !has_uppercase || !has_digit || !has_special {
            errors.add("password", ValidationError::new(
                "password must contain at least one uppercase letter, one lowercase letter, one number, and one special character (@$!%*?&)"
            ));
        }

        // Validate password match
        if self.password != self.password_verify {
            errors.add(
                "password_verify",
                ValidationError::new("passwords do not match"),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl CreateUserDto {
    pub fn is_password_common(&self) -> bool {
        COMMON_PASSWORDS.contains(self.password.as_str())
    }

    pub fn sanitize(&mut self) {
        self.username = self
            .username
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        self.email = self.email.to_lowercase();
    }
}
