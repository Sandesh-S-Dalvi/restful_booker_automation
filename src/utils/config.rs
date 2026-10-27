use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            base_url: env::var("BOOKER_BASE_URL")
                .unwrap_or_else(|_| "https://restful-booker.herokuapp.com".to_string()),
            username: env::var("BOOKER_USERNAME").unwrap_or_else(|_| "admin".to_string()),
            password: env::var("BOOKER_PASSWORD").unwrap_or_else(|_| "password123".to_string()),
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
