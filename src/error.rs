#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(#[from] crate::client::ApiError),
    #[error("Rate limited")]
    RateLimited,
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
}
