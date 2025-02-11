use mongodb::{
    bson::doc,
    options::ClientOptions,
    Client, Collection, Database as MongoDatabase,
};
use crate::{error::AppError, models::user::User};

#[derive(Clone, Debug)]
pub struct Database {
    pub client: Client,
    pub db: MongoDatabase,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, AppError> {
        let client_options = ClientOptions::parse(database_url)
            .await
            .map_err(AppError::DatabaseError)?;
            
        let client = Client::with_options(client_options)
            .map_err(AppError::DatabaseError)?;
            
        let db = client.database("gamie");
        
        // Ping the database to ensure connection is valid
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .map_err(AppError::DatabaseError)?;
            
        Ok(Database { client, db })
    }
    
    pub fn get_users_collection(&self) -> Collection<User> {
        self.db.collection("users")
    }
} 