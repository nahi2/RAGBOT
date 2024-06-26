use dotenv::dotenv;
use std::env;
use reqwest;
use reqwest::header::{ACCEPT};
use serde_json;
use serde_json::Value;

#[derive(Debug)]
pub struct ConfCreds {
    domain: String,
    username: String,
    password: String,
}

impl ConfCreds {
    pub fn set_creds() -> Result<Self, String> {
        dotenv().ok();

        let domain = env::var("CONFLUENCE_DOMAIN").map_err(|_| "Domain not set.".to_string())?;
        let username = env::var("CONFLUENCE_USERNAME").map_err(|_| "Username not set.".to_string())?;
        let password = env::var("CONFLUENCE_PASSWORD").map_err(|_| "Password not set.".to_string())?;

        Ok(ConfCreds {
            domain,
            username,
            password,
        })
    }

    pub fn get_domain(&self) -> &str {
        &self.domain
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }

     pub async fn get_pages(&self) -> Result<Value, String> {
        let client = reqwest::Client::new();
        let url = format!("https://{}/wiki/api/v2/pages?body-format=storage", self.get_domain());

        let response = client
            .get(&url)
            .basic_auth(self.get_username(), Some(self.get_password()))
            .header(ACCEPT, "application/json")
            .send()
            .await
            .map_err(|e| e.to_string())?;

         if response.status().is_success() {
             let body = response.text().await.map_err(|e| e.to_string())?;
             match serde_json::from_str(&body) {
                 Ok(v) => Ok(v),
                 Err(e) => Err(format!("JSON Conversion failed: {}", e))
             }
         } else {
             let status = response.status();
             Err(format!("Request failed with status: {}", status))
         }
     }
}