use std::env;
use dotenv::dotenv;
use mongodb::{bson, Client};
use serde_json::Value;

pub struct MongoDBConfig {
    url: String,
    database: String,
    collection: String
}

impl MongoDBConfig{
    pub fn create_config() -> Result<Self, String> {
        dotenv().ok();

        let url = env::var("MONGODB_CLIENT").map_err(|_| "Domain not set.".to_string())?;
        let database = env::var("MONGODB_DATABASE").map_err(|_| "Username not set.".to_string())?;
        let collection = env::var("MONGODB_COLLECTION").map_err(|_| "Password not set.".to_string())?;

        Ok(MongoDBConfig {
            url,
            database,
            collection,
        })
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_database(&self) -> &str {
        &self.database
    }

    pub fn get_collection(&self) -> &str {
        &self.collection
    }

    pub async fn insert_page(&self, json_pages: Value) -> Result<(), String> {
        let client = Client::with_uri_str(&self.get_url())
            .await
            .map_err(|e| format!("Failed to create client: {:?}", e))?;

        let db = client.database(&self.get_database());
        let collection = db.collection(&self.get_collection());

        let bson_doc = match bson::to_document(&json_pages) {
            Ok(doc) => doc,
            Err(e) => {
                return Err(format!("Failed to convert JSON to BSON: {:?}", e))
            }
        };

        collection.insert_one(bson_doc, None).await
            .map_err(|e| format!("Failed to insert page: {:?}", e))?;

        Ok(())
    }
}
