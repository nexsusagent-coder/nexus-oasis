//! Error types for sentient_vision

use thiserror::Error;

/// Vision error type
#[derive(Error, Debug)]
pub enum VisionError {
    /// Image processing error
    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    /// Invalid image format
    #[error("Invalid image format: {0}")]
    InvalidFormat(String),

    /// Image too large
    #[error("Image too large: {0} bytes (max: {1})")]
    TooLarge(usize, usize),

    /// OCR error
    #[error("OCR error: {0}")]
    Ocr(String),

    /// Vision model error
    #[error("Vision model error: {0}")]
    Model(String),

    /// API error
    #[error("API error: {0}")]
    Api(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Timeout
    #[error("Request timed out after {0} seconds")]
    Timeout(u64),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Provider not available
    #[error("Vision provider '{0}' is not available")]
    ProviderNotAvailable(String),

    /// Feature not supported
    #[error("Feature '{0}' is not supported by this provider")]
    FeatureNotSupported(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    Encoding(String),

    /// Decode error
    #[error("Decode error: {0}")]
    Decode(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Base64 error
    #[error("Base64 error: {0}")]
    Base64(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type alias for Vision operations
pub type Result<T> = std::result::Result<T, VisionError>;

impl VisionError {
    /// Create an image processing error
    pub fn image(msg: impl Into<String>) -> Self {
        Self::ImageProcessing(msg.into())
    }

    /// Create an invalid format error
    pub fn invalid_format(format: impl Into<String>) -> Self {
        Self::InvalidFormat(format.into())
    }

    /// Create an OCR error
    pub fn ocr(msg: impl Into<String>) -> Self {
        Self::Ocr(msg.into())
    }

    /// Create a model error
    pub fn model(msg: impl Into<String>) -> Self {
        Self::Model(msg.into())
    }

    /// Create an API error
    pub fn api(msg: impl Into<String>) -> Self {
        Self::Api(msg.into())
    }

    /// Create a network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /// Create a config error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a provider not available error
    pub fn provider_not_available(provider: impl Into<String>) -> Self {
        Self::ProviderNotAvailable(provider.into())
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Network(_) | Self::RateLimit(_) | Self::Timeout(_)
        )
    }

    /// Check if error is due to rate limiting
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, Self::RateLimit(_))
    }

    /// Check if error is due to timeout
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }
}

impl From<base64::DecodeError> for VisionError {
    fn from(e: base64::DecodeError) -> Self {
        Self::Base64(e.to_string())
    }
}

impl From<image::ImageError> for VisionError {
    fn from(e: image::ImageError) -> Self {
        Self::ImageProcessing(e.to_string())
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for VisionError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Self::Timeout(30)
        } else if e.is_connect() {
            Self::Network(e.to_string())
        } else if e.is_status() {
            Self::Api(e.to_string())
        } else {
            Self::Network(e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = VisionError::image("Failed to decode");
        assert!(err.to_string().contains("Failed to decode"));
    }

    #[test]
    fn test_error_retryable() {
        let err = VisionError::network("Connection failed");
        assert!(err.is_retryable());

        let err = VisionError::model("Invalid model");
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_error_from_image() {
        let image_err = image::ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found"
        ));
        let vision_err: VisionError = image_err.into();
        assert!(matches!(vision_err, VisionError::ImageProcessing(_)));
    }
}
