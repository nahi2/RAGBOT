mod confluence;

use mongodb::{
    //bson::{Document, doc},
    Client,
    // Collection
};
use std::result;

type Result<T> = result::Result<T, ()>;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = Client::with_uri_str("mongdb://localhost:27017")
        .await
        .map_err(|e| {
            eprintln!("Failed to create client: {:?}", e);
        })?;
    Ok(())
}
