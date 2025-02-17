use crate::error::{AppError, ErrorResponse};
use crate::models::user::User;
use mongodb::{bson::doc, Client, Collection, Database as MongoDatabase};

#[derive(Debug, Clone)]
pub struct Database {
    db: MongoDatabase,
}

impl Database {
    pub async fn connect(uri: &str) -> Result<Self, AppError> {
        let client = Client::with_uri_str(uri).await.map_err(|e| {
            AppError::DatabaseError(ErrorResponse {
                code: "DATABASE_CONNECTION_ERROR".to_string(),
                message: format!("Failed to connect to database: {}", e),
                field: None,
            })
        })?;

        let db = client.database("gamie");

        // Ping the database to ensure connection is valid
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .map_err(|e| {
                AppError::DatabaseError(ErrorResponse {
                    code: "DATABASE_CONNECTION_ERROR".to_string(),
                    message: format!("Failed to ping database: {}", e),
                    field: None,
                })
            })?;

        Ok(Self { db })
    }

    pub fn get_users_collection(&self) -> Collection<User> {
        self.db.collection("users")
    }
}
