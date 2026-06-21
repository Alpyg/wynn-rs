use crate::{modules::guild::Guild, prelude::*};
use reqwest::{Client, header};
use serde::Deserialize;
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

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
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    client: Client,
    base_url: String,
}

pub struct WynncraftClientBuilder {
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
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn build(self) -> Result<WynncraftClient, reqwest::Error> {
        let client = Client::builder().build()?;
        Ok(WynncraftClient {
            inner: Arc::new(Inner {
                client,
                base_url: self.base_url,
            }),
        })
    }
}

impl WynncraftClient {
    pub fn builder() -> WynncraftClientBuilder {
        WynncraftClientBuilder::new()
    }

    pub fn as_user(&self, token: impl Into<String>) -> WynncraftSession {
        WynncraftSession {
            client: self.clone(),
            token: Some(token.into()),
        }
    }

    pub fn anonymous(&self) -> WynncraftSession {
        WynncraftSession {
            client: self.clone(),
            token: None,
        }
    }
}

pub struct WynncraftSession {
    client: WynncraftClient,
    token: Option<String>,
}

impl WynncraftSession {
    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, WynncraftError> {
        let inner = &self.client.inner;
        let mut request = inner.client.get(format!("{}/{}", inner.base_url, path));

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        match response.status().as_u16() {
            200 => Ok(response.json::<T>().await?),
            429 => Err(WynncraftError::RateLimited),
            _ => Err(WynncraftError::Api(
                response.json::<WynncraftApiError>().await?,
            )),
        }
    }

    pub async fn player(
        &self,
        uuid: Uuid,
        full_result: bool,
    ) -> Result<PlayerProfile, WynncraftError> {
        if full_result {
            self.get(&format!("player/{uuid}?fullResult")).await
        } else {
            self.get(&format!("player/{uuid}")).await
        }
    }

    pub async fn characters(&self, uuid: Uuid) -> Result<CharacterSummaries, WynncraftError> {
        self.get(&format!("player/{uuid}/characters")).await
    }

    pub async fn character(
        &self,
        uuid: Uuid,
        character: Uuid,
    ) -> Result<Character, WynncraftError> {
        self.get(&format!("player/{uuid}/characters/{character}"))
            .await
    }

    pub async fn guild_by_prefix(&self, prefix: &str) -> Result<Guild, WynncraftError> {
        self.get(&format!("guild/prefix/{prefix}")).await
    }
}
