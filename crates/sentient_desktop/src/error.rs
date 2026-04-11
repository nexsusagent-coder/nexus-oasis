// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Desktop Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DesktopError>;

/// Desktop automation errors
#[derive(Debug, Error)]
pub enum DesktopError {
    #[error("Screen capture failed: {0}")]
    ScreenCaptureFailed(String),

    #[error("Mouse operation failed: {0}")]
    MouseFailed(String),

    #[error("Keyboard operation failed: {0}")]
    KeyboardFailed(String),

    #[error("Window not found: {0}")]
    WindowNotFound(String),

    #[error("Element not found on screen")]
    ElementNotFound,

    #[error("Timeout waiting for element")]
    Timeout,

    #[error("Invalid coordinates: {0}")]
    InvalidCoordinates(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image error: {0}")]
    ImageError(String),
}
