use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
}