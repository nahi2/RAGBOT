mod confluence;

use mongodb::{
    //bson::{Document, doc},
    Client,
    // Collection
};
use std::result;
use crate::confluence::ConfCreds;

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

    println!("creds: {:?}", conf_creds);
    let _ = Client::with_uri_str("mongdb://localhost:27017")
        .await
        .map_err(|e| {
            eprintln!("Failed to create client: {:?}", e);
        })?;


    Ok(())
}
