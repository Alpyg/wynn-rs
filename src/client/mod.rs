use std::fmt;

use reqwest::{Client, header};
use serde::Deserialize;
use uuid::Uuid;

use crate::{modules::guild::Guild, prelude::*};

const BASE_URL: &str = "https://api.wynncraft.com/v3";

#[derive(Debug, Deserialize)]
pub struct WynncraftApiError {
    pub error: String,
    pub detail: String,
    pub code: u16,
}

impl fmt::Display for WynncraftApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}): {}", self.error, self.code, self.detail)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WynncraftError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(WynncraftApiError),
    #[error("Rate limited")]
    RateLimited,
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct WynncraftClient {
    client: Client,
    base_url: String,
}

pub struct WynncraftClientBuilder {
    token: Option<String>,
    base_url: String,
}

impl Default for WynncraftClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WynncraftClientBuilder {
    pub fn new() -> Self {
        Self {
            token: None,
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn build(self) -> Result<WynncraftClient, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        if let Some(token) = self.token {
            let mut value =
                header::HeaderValue::from_str(&token).expect("invalid token header value");
            value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, value);
        }

        let client = Client::builder().default_headers(headers).build()?;

        Ok(WynncraftClient {
            client,
            base_url: self.base_url,
        })
    }
}

impl WynncraftClient {
    pub fn builder() -> WynncraftClientBuilder {
        WynncraftClientBuilder::new()
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, WynncraftError> {
        let response = self
            .client
            .get(format!("{}/{}", self.base_url, path))
            .send()
            .await?;

        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await?),
            429 => Err(WynncraftError::RateLimited),
            _ => Err(WynncraftError::Api(
                response.json::<WynncraftApiError>().await?,
            )),
        }
    }

    pub async fn player(&self, username: &str) -> Result<PlayerProfile, WynncraftError> {
        self.get(&format!("player/{username}")).await
    }

    pub async fn characters(&self, username: &str) -> Result<CharacterSummaries, WynncraftError> {
        self.get(&format!("player/{username}/characters")).await
    }

    pub async fn character(
        &self,
        username: &str,
        character: Uuid,
    ) -> Result<Character, WynncraftError> {
        self.get(&format!("player/{username}/characters/{character}"))
            .await
    }

    pub async fn guild_by_prefix(&self, prefix: &str) -> Result<Guild, WynncraftError> {
        self.get(&format!("guild/prefix/{prefix}")).await
    }
}
