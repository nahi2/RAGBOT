mod confluence;

use mongodb::{bson, Client};
use std::result;
use crate::confluence::ConfCreds;
use serde_json::{Value};


type Result<T> = result::Result<T, ()>;

#[tokio::main]
async fn main() -> Result<()> {
    let conf_creds = match ConfCreds::set_creds() {
        Ok(creds) => {
            creds
        }
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };

    let response = match conf_creds.get_pages().await {
        Ok(response) => {
            response
        }
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };

    let json_pages: Value = match serde_json::from_str(&response) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return Ok(());
        }
    };

    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .map_err(|e| {
            eprintln!("Failed to create client: {:?}", e);
        })?;

    let db = client.database("ConfDatabase");
    let collection = db.collection("ConfContent");
    
    let bson_doc = match bson::to_document(&json_pages) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Failed to convert JSON to BSON: {:?}", e);
            return Ok(());
        }
    };

    let _ = collection.insert_one(bson_doc, None).await.map_err(|e1| {
        eprintln!("{}", e1)
    });

    Ok(())
}
