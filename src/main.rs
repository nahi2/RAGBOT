mod confluence;
mod db_config;

use std::{io};
use actix_web::{App, HttpResponse, HttpServer, get};
use serde_json::Value;
use crate::confluence::ConfCreds;
use crate::db_config::MongoDBConfig;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(setup).service(ids))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/setup")]
async fn setup() -> HttpResponse {
    let conf_creds = match ConfCreds::set_creds() {
        Ok(creds) => {
            creds
        }
        Err(e) => {
            eprintln!("failed to get confluence credentials: {}", e);
            return HttpResponse::InternalServerError().finish()
        }
    };

    let response = match conf_creds.get_pages().await {
        Ok(response) => {
            response
        }
        Err(e) => {
            eprintln!("{}", e);
            return HttpResponse::Unauthorized().finish()
        }
    };

    let response_parsed = match response["results"].as_array() {
        None => {
            eprintln!("failed to parse confluence results");
            return HttpResponse::InternalServerError().finish()
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
            return HttpResponse::InternalServerError().finish()
        }
    };

    match mongodb_config.insert_pages(response_parsed).await {
        Err(e) => {
            eprintln!("{e}");
            HttpResponse::InternalServerError().finish()
        }
        _ => {
            HttpResponse::Ok().finish()
        }
    }
}

#[get("/ids")]
async fn ids() -> HttpResponse {
    let mongodb_config = match MongoDBConfig::create_config() {
        Ok(value) => {
            value
        }
        Err(e) => {
            eprintln!("{}", e);
            return HttpResponse::InternalServerError().finish()
        }
    };

    mongodb_config.get_ids().await.unwrap();

    HttpResponse::Ok().finish()
}

