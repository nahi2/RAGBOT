mod confluence;
mod db_config;
mod qdrant_config;

use crate::confluence::ConfCreds;
use crate::db_config::MongoDBConfig;

#[tokio::main]
async fn main() {
    let conf_creds = match ConfCreds::set_creds() {
        Ok(creds) => {
            creds
        }
        Err(e) => {
            eprintln!("failed to get confluence credentials: {}", e);
            return
        }
    };

    let response = match conf_creds.get_pages().await {
        Ok(response) => {
            response
        }
        Err(e) => {
            eprintln!("{}", e);
            return
        }
    };

    let response_parsed = match response["results"].as_array() {
        None => {
            eprintln!("failed to parse confluence results");
            return
        }
        Some(val) => {
            val
        }
    };


    let mongodb_config = match MongoDBConfig::create_config() {
        Ok(value) => {
            value
        }
        Err(e) => {
            eprintln!("{}", e);
            return
        }
    };

    if let Err(e) = mongodb_config.insert_pages(response_parsed).await {
        eprintln!("Failed to insert pages: {}", e);
        return;
    }

    if let Err(e) = mongodb_config.get_content().await {
        eprintln!("Failed to get document IDs: {}", e);
        return;
    }
}
