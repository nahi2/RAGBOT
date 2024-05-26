use dotenv::dotenv;
use std::env;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION};

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

     pub async fn get_pages(&self) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("https://{}/wiki/api/v2/pages?body-format=storage", &self.domain);

        let auth = format!("{}:{}", &self.username, &self.password);
        let auth = format!("Basic {}", base64::encode(auth));

        let response = client
            .get(&url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, auth)
            .send()
            .await?;

         Ok(response)
    }
}