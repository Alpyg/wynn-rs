use crate::{modules::guild::Guild, *};
use reqwest::Client;
use std::fmt;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

const BASE_URL: &str = "https://api.wynncraft.com/v3";

#[derive(Debug, Error)]
pub struct ApiError {
    pub error: String,
    pub detail: String,
    pub code: u16,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}): {}", self.error, self.code, self.detail)
    }
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
    async fn handle<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, Error> {
        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        match status {
            200 => {
                let de = &mut serde_json::Deserializer::from_slice(&bytes);
                serde_path_to_error::deserialize(de).map_err(|e| {
                    tracing::error!(path = %e.path(), error = %e.inner(), "deserialize response");
                    Error::Deserialize(e.into_inner())
                })
            }
            429 => Err(Error::RateLimited),
            _ => {
                let body = String::from_utf8_lossy(&bytes).into_owned();
                Err(Error::Api(ApiError {
                    error: status.to_string(),
                    detail: body,
                    code: status,
                }))
            }
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let inner = &self.client.inner;
        let mut request = inner.client.get(format!("{}/{}", inner.base_url, path));
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        let response = request.send().await?;
        self.handle(response).await
    }

    pub async fn oauth_token<'a>(
        &self,
        req: AccessTokenRequest<'a>,
    ) -> Result<AccessTokenResponse, Error> {
        let inner = &self.client.inner;
        let mut form: Vec<(&str, &str)> = vec![
            ("grant_type", &req.grant_type),
            ("code", &req.code),
            ("redirect_uri", &req.redirect_uri),
            ("client_id", &req.client_id),
        ];

        match &req.auth {
            ClientAuth::ClientSecret(s) => form.push(("client_secret", s)),
            ClientAuth::CodeVerifier(v) => form.push(("code_verifier", v)),
        }
        let response = inner
            .client
            .post(format!("{}/oauth/token", inner.base_url))
            .form(&form)
            .send()
            .await?;
        self.handle(response).await
    }

    pub async fn player(&self, uuid: Uuid, full_result: bool) -> Result<PlayerProfile, Error> {
        if full_result {
            self.get(&format!("player/{uuid}?fullResult")).await
        } else {
            self.get(&format!("player/{uuid}")).await
        }
    }

    pub async fn characters(&self, uuid: Uuid) -> Result<CharacterSummaries, Error> {
        self.get(&format!("player/{uuid}/characters")).await
    }

    pub async fn character(&self, uuid: Uuid, character: Uuid) -> Result<Character, Error> {
        self.get(&format!("player/{uuid}/characters/{character}"))
            .await
    }

    pub async fn guild_by_prefix(&self, prefix: &str) -> Result<Guild, Error> {
        self.get(&format!("guild/prefix/{prefix}")).await
    }

    pub async fn territories(&self) -> Result<Territories, Error> {
        self.get(&format!("guild/list/territory")).await
    }
}
