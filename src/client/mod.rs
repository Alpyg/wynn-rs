mod cache;

use std::{fmt, sync::Arc};

use reqwest::{Client, header};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    client::cache::{Bucket, BucketLimiter, ResponseCache, make_cache},
    prelude::*,
};

const BASE_URL: &str = "https://api.wynncraft.com/v3";
const AUTH_RPM: u32 = 120;
const ANON_RPM: u32 = 50;

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

pub struct WynncraftClient {
    client: Client,
    base_url: String,

    player_cache: ResponseCache,
    guild_cache: ResponseCache,

    player_limiter: Arc<BucketLimiter>,
    guild_limiter: Arc<BucketLimiter>,
}

pub struct WynncraftClientBuilder {
    token: Option<String>,
    base_url: String,

    player_ttl_secs: u64,
    guild_ttl_secs: u64,
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
            player_ttl_secs: 300,
            guild_ttl_secs: 300,
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

    pub fn with_player_ttl(mut self, secs: u64) -> Self {
        self.player_ttl_secs = secs;
        self
    }

    pub fn with_guild_ttl(mut self, secs: u64) -> Self {
        self.guild_ttl_secs = secs;
        self
    }
    pub fn build(self) -> Result<WynncraftClient, reqwest::Error> {
        let authenticated = self.token.is_some();
        let rpm = if authenticated { AUTH_RPM } else { ANON_RPM };

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

            player_cache: make_cache(self.player_ttl_secs),
            guild_cache: make_cache(self.guild_ttl_secs),

            player_limiter: Arc::new(BucketLimiter::new(rpm)),
            guild_limiter: Arc::new(BucketLimiter::new(rpm)),
        })
    }
}

impl WynncraftClient {
    pub fn builder() -> WynncraftClientBuilder {
        WynncraftClientBuilder::new()
    }

    fn cache_for(&self, bucket: Bucket) -> &ResponseCache {
        match bucket {
            Bucket::Player => &self.player_cache,
            Bucket::Guild => &self.guild_cache,
        }
    }

    fn limiter_for(&self, bucket: Bucket) -> &BucketLimiter {
        match bucket {
            Bucket::Player => &self.player_limiter,
            Bucket::Guild => &self.guild_limiter,
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        bucket: Bucket,
        path: &str,
    ) -> Result<T, WynncraftError> {
        let cache = self.cache_for(bucket);
        let key = path.to_string();

        if let Some(cached) = cache.get(&key).await {
            return Ok(serde_json::from_value((*cached).clone())?);
        }

        self.limiter_for(bucket).acquire().await?;

        let response = self
            .client
            .get(format!("{}/{}", self.base_url, path))
            .send()
            .await?;

        match response.status().as_u16() {
            200 => {
                let value: serde_json::Value = response.json().await?;
                let result = serde_json::from_value::<T>(value.clone())?;
                cache.insert(key, Arc::new(value)).await;
                Ok(result)
            }
            429 => Err(WynncraftError::RateLimited),
            _ => Err(WynncraftError::Api(
                response.json::<WynncraftApiError>().await?,
            )),
        }
    }

    pub async fn player(&self, username: &str) -> Result<PlayerProfile, WynncraftError> {
        self.get(Bucket::Player, &format!("player/{username}"))
            .await
    }

    pub async fn characters(&self, username: &str) -> Result<CharacterSummaries, WynncraftError> {
        self.get(Bucket::Player, &format!("player/{username}/characters"))
            .await
    }

    pub async fn character(
        &self,
        username: &str,
        character: Uuid,
    ) -> Result<Character, WynncraftError> {
        self.get(
            Bucket::Player,
            &format!("player/{username}/characters/{character}"),
        )
        .await
    }
}
