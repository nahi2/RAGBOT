mod confluence;
mod db_config;

use std::result;
use actix_web::{App, HttpServer};
use crate::confluence::ConfCreds;
use crate::db_config::MongoDBConfig;


type Result<T> = result::Result<T, ()>;

#[actix_web::main]
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

    let mongodb_config = match MongoDBConfig::create_config() {
        Ok(value) => {
            value
        }
        Err(e) => {
            eprintln!("{}", e);
            return Ok(())
        }
    };

    let _ = mongodb_config.insert_page(response).await;

    HttpServer::new(move || {
        App::new()
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
