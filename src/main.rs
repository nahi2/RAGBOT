mod confluence;
mod db_config;
mod qdrant_config;
mod open_ai_config;

use std::error::Error;
use std::future::Future;
use std::process;
use mongodb::bson::Bson;
use qdrant_client::qdrant::PointStruct;
use crate::confluence::ConfCreds;
use crate::db_config::MongoDBConfig;
use crate::open_ai_config::gen_embeddings;

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

    let content = match mongodb_config.get_content().await {
        Ok(ok) => {ok}
        Err(e) => {
            eprintln!("{}", format!("{e}"));
            return;
        }
    };
    
    let _collection = qdrant_config::create_collection().await.map_err(|e1| {
        eprintln!("{e1}");
        process::exit(1);
    });
    
    let mut point_structs: Vec<PointStruct> = vec![];

    for doc in content {
        match doc.get("_id") {
            Some(Bson::ObjectId(id)) => {
                match doc.get("body") {
                    Some(Bson::Document(body)) => {
                        match body.get("storage") {
                            Some(Bson::Document(storage)) => {
                                match storage.get("value") {
                                    Some(Bson::String(value)) => {
                                        match gen_embeddings(value).await {
                                            Ok(embedding) => {
                                                let point_id = MongoDBConfig::hash_object_id(id);
                                                point_structs.push(PointStruct::new(point_id, embedding, Default::default()));
                                            },
                                            Err(e) => {
                                                eprintln!("Failed to generate embeddings for document with id {}: {}", id, e);
                                                return;
                                            }
                                        }
                                    },
                                    _ => {
                                        eprintln!("Failed to get 'value' from 'storage' for document with id {}", id);
                                        return;
                                    }
                                }
                            },
                            _ => {
                                eprintln!("Failed to get 'storage' from 'body' for document with id {}", id);
                                return;
                            }
                        }
                    },
                    _ => {
                        eprintln!("Failed to get 'body' for document with id {}", id);
                        return;
                    }
                }
            },
            _ => {
                eprintln!("Failed to get '_id' from document");
                return;
            }
        }
    }

    let _ = qdrant_config::upload_points_store(point_structs).await.map_err(|e2| {
        eprintln!("{e2}");
        process::exit(1);
    });
}
