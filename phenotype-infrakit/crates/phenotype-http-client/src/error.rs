//! Error types for HTTP client

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in HTTP client
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP error
    #[error("HTTP error: {status} - {message}")]
    Http {
        /// HTTP status code
        status: u16,
        /// Error message
        message: String,
    },

    /// Network error
    #[error("network error: {0}")]
    Network(String),

    /// Timeout error
    #[error("timeout: {operation} after {duration:?}")]
    Timeout {
        /// Operation that timed out
        operation: String,
        /// Duration
        duration: std::time::Duration,
    },

    /// Request building error
    #[error("request error: {0}")]
    Request(String),

    /// Response error
    #[error("response error: {0}")]
    Response(String),

    /// Serialization error
    #[error("serialization error: {0}")]
    Serialization(String),

    /// Invalid URI
    #[error("invalid URI: {0}")]
    InvalidUri(String),

    /// Redirect error
    #[error("redirect error: {0}")]
    Redirect(String),

    /// Connection error
    #[error("connection error: {0}")]
    Connection(String),

    /// Pool error
    #[error("pool error: {0}")]
    Pool(String),

    /// Retry exhausted
    #[error("retry exhausted after {attempts} attempts: {message}")]
    RetryExhausted {
        /// Number of attempts
        attempts: u32,
        /// Error message
        message: String,
    },

    /// Interceptor error
    #[error("interceptor error: {0}")]
    Interceptor(String),

    /// Unknown error
    #[error("unknown error: {0}")]
    Unknown(String),

    /// Adapter not available
    #[error("adapter not available: {0}")]
    AdapterNotAvailable(String),
}

impl Error {
    /// Create a request error
    pub fn request(msg: impl Into<String>) -> Self {
        Self::Request(msg.into())
    }

    /// Create a network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /// Create a serialization error
    pub fn serialization(msg: impl Into<String>) -> Self {
        Self::Serialization(msg.into())
    }

    /// Create a connection error
    pub fn connection(msg: impl Into<String>) -> Self {
        Self::Connection(msg.into())
    }

    /// Create a pool error
    pub fn pool(msg: impl Into<String>) -> Self {
        Self::Pool(msg.into())
    }

    /// Create a redirect error
    pub fn redirect(msg: impl Into<String>) -> Self {
        Self::Redirect(msg.into())
    }

    /// Create an interceptor error
    pub fn interceptor(msg: impl Into<String>) -> Self {
        Self::Interceptor(msg.into())
    }

    /// Create a retry exhausted error
    pub fn retry_exhausted(attempts: u32, message: impl Into<String>) -> Self {
        Self::RetryExhausted {
            attempts,
            message: message.into(),
        }
    }
}
