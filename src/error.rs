use thiserror::Error;

/// SDK 错误
#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Text moderation error: {0}")]
    TextModerationError(TextModerationError),
    #[error("Image moderation error: {0}")]
    ImageModerationError(ImageModerationError)
}

/// 文本审核错误
#[derive(Debug, Error)]
pub enum TextModerationError {
    #[error("service is not set")]
    ServiceNotSet,
    #[error("content is not set")]
    ContentNotSet
}

/// 图片审核错误
#[derive(Debug, Error)]
pub enum ImageModerationError {
    #[error("service is not set")]
    ServiceNotSet,
    #[error("image is not set")]
    ImageNotSet,
}