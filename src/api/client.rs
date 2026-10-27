use crate::models::auth::{AuthRequest, AuthResponse};
use crate::models::booking::{Booking, BookingId, BookingResponse};
use crate::utils::config::Config;
use anyhow::{Context, Result};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use tracing::{debug, instrument};

#[derive(Clone)]
pub struct BookerClient {
    http: Client,
    base_url: String,
    token: Option<String>,
}

impl BookerClient {
    pub fn new(config: Config) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            base_url: config.base_url,
            token: None,
        }
    }

    pub async fn authenticate(&mut self, username: &str, password: &str) -> Result<AuthResponse> {
        let body = AuthRequest {
            username: username.to_string(),
            password: password.to_string(),
        };
        let resp = self
            .http
            .post(format!("{}/auth", self.base_url))
            .json(&body)
            .send()
            .await
            .context("auth request failed")?;
        let auth: AuthResponse = self.handle(resp).await?;
        self.token = Some(auth.token.clone());
        Ok(auth)
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    fn auth_header(&self) -> Option<(&str, String)> {
        self.token.as_ref().map(|t| ("Cookie", format!("token={}", t)))
    }

    async fn handle<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T> {
        let status = resp.status();
        let text = resp.text().await.context("failed to read body")?;
        debug!(status = %status, body = %text, "response");
        if !status.is_success() {
            anyhow::bail!("HTTP {} : {}", status, text);
        }
        serde_json::from_str(&text).context("deserialize failed")
    }

    #[instrument(skip(self))]
    pub async fn ping(&self) -> Result<bool> {
        let resp = self.http.get(format!("{}/ping", self.base_url)).send().await?;
        Ok(resp.status() == StatusCode::CREATED || resp.status().is_success())
    }

    #[instrument(skip(self, booking))]
    pub async fn create_booking(&self, booking: &Booking) -> Result<BookingResponse> {
        let resp = self
            .http
            .post(format!("{}/booking", self.base_url))
            .json(booking)
            .send()
            .await?;
        self.handle(resp).await
    }

    #[instrument(skip(self))]
    pub async fn get_booking(&self, id: i64) -> Result<Booking> {
        let resp = self
            .http
            .get(format!("{}/booking/{}", self.base_url, id))
            .send()
            .await?;
        self.handle(resp).await
    }

    #[instrument(skip(self))]
    pub async fn get_booking_ids(&self) -> Result<Vec<BookingId>> {
        let resp = self.http.get(format!("{}/booking", self.base_url)).send().await?;
        self.handle(resp).await
    }

    #[instrument(skip(self, booking))]
    pub async fn update_booking(&self, id: i64, booking: &Booking) -> Result<Booking> {
        let mut req = self
            .http
            .put(format!("{}/booking/{}", self.base_url, id))
            .json(booking);
        if let Some((name, value)) = self.auth_header() {
            req = req.header(name, value);
        }
        let resp = req.send().await?;
        self.handle(resp).await
    }

    #[instrument(skip(self))]
    pub async fn delete_booking(&self, id: i64) -> Result<()> {
        let mut req = self.http.delete(format!("{}/booking/{}", self.base_url, id));
        if let Some((name, value)) = self.auth_header() {
            req = req.header(name, value);
        }
        let resp = req.send().await?;
        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("delete failed: {}", text);
        }
        Ok(())
    }
}
