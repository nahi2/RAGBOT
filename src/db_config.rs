use std::env;
use std::fmt::format;
use std::hash::{DefaultHasher, Hash, Hasher};
use dotenv::dotenv;
use futures::TryStreamExt;
use mongodb::{bson, Client, Collection};
use mongodb::bson::{doc, Document};
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

    async fn get_collection_handle(&self) -> Result<Collection<Document>, String> {
        let client = Client::with_uri_str(&self.get_url())
            .await
            .map_err(|e| format!("Failed to create client: {:?}", e))?;
        let db = client.database(&self.get_database());
        Ok(db.collection(&self.get_collection()))
    }

    pub async fn insert_pages(&self, json_pages: &Vec<Value>) -> Result<(), String> {

        let collection = match self.get_collection_handle().await {
            Ok(collection) => {collection}
            Err(e) => {return Err(format!("failed to get handle {e}"))?}
        };

        let mut documents = vec![];
        for page in json_pages {
            match bson::to_document(&page) {
                Ok(doc) => documents.push(doc),
                Err(e) => {
                    return Err(format!("Failed to convert JSON to BSON: {:?}", e))?
                }
            };
        }

        let _ = collection.insert_many(documents, None).await
            .map_err(|e| format!("Failed to insert page: {:?}", e)).map_err(|e1| {
            return format!("Failed to insert page: {:?}", e1);
        });

        Ok(())
    }

    pub async fn get_content(&self) -> Result<Vec<Document>, String> {
        // Get a handle to a collection in the database.
        let collection = self.get_collection_handle().await?;

        // Define the projection document.
        let projection = doc! { "_id": 1, "body.storage.value": 1 };

        let find_options = mongodb::options::FindOptions::builder()
            .projection(projection)
            .build();

        // Perform the find query.
        let mut cursor = collection.find(None, find_options).await
            .map_err(|e| format!("Failed to execute find query: {:?}", e))?;

        let mut results_vec = vec![];
        while let Some(result) = cursor.try_next().await
            .map_err(|e| format!("Failed to iterate cursor: {:?}", e))? {
            results_vec.push(result)
        };
        return Ok(results_vec)
    }

    pub(crate) fn hash_object_id(object_id: &bson::oid::ObjectId) -> u64 {
        let mut hasher = DefaultHasher::new();
        object_id.hash(&mut hasher);
        hasher.finish()
    }
}
