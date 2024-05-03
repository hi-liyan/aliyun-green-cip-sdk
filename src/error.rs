use thiserror::Error;

/// SDK 错误
#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Bad request parameters: {0}")]
    BadParams(TextModerationError)
}

/// 文本审核错误
#[derive(Debug, Error)]
pub enum TextModerationError {
    #[error("service is not set")]
    ServiceNotSet,
    #[error("content is not set")]
    ContentNotSet
}