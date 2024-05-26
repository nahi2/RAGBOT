use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct ConfCreds {
    domain: String,
    username: String,
    password: String,
}

impl ConfCreds {
    pub fn set_creds() -> Result<Self, String> {
        dotenv().ok();

        let domain = env::var("DOMAIN").map_err(|_| "Domain not set.".to_string())?;
        let username = env::var("CONFLUENCE_USERNAME").map_err(|_| "Username not set.".to_string())?;
        let password = env::var("CONFLUENCE_TOKEN").map_err(|_| "Password not set.".to_string())?;

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

    pub fn get_pages(&self) -> Result<(),()> {

    }
}