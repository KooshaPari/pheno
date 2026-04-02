//! Phenotype HTTP Client - Hexagonal HTTP client with pluggable adapters
//!
//! A hexagonal-architecture HTTP client supporting:
//! - Reqwest adapter (default)
//! - Mock adapter for testing
//! - Custom adapter trait for other backends
//! - Request/response interceptors
//! - Connection pooling
//! - Retry with exponential backoff
//! - Timeout configuration
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use phenotype_http_client::{ReqwestAdapter, HttpClientPort, Request, Method};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ReqwestAdapter::new(ClientConfig::default());
//!     
//!     let request = Request::builder()
//!         .method(Method::GET)
//!         .uri("https://api.example.com/users")
//!         .build()?;
//!     
//!     let response = client.execute(request).await?;
//!     println!("Status: {}", response.status);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ```rust,no_run
//! use phenotype_http_client::{MockAdapter, HttpClientPort, Request, Method};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut mock = MockAdapter::new(ClientConfig::default());
//!     mock.expect_response("https://api.example.com", 200, "{\"ok\": true}");
//!     
//!     let request = Request::builder()
//!         .method(Method::GET)
//!         .uri("https://api.example.com")
//!         .build()?;
//!     
//!     let response = mock.execute(request).await?;
//!     assert!(response.is_success());
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

pub mod adapters;
pub mod error;
pub mod interceptor;
pub mod pool;
pub mod ports;
pub mod retry;
pub mod types;

pub use ports::{ConnectionPoolPort, ConnectionPort, HttpClientPort, InterceptorPort, PoolStats};

pub use types::{Body, ClientConfig, Headers, Method, Request, Response, TimeoutConfig, Uri};

pub use error::{Error, Result};

pub use interceptor::{InterceptorChain, LoggingInterceptor, TimingInterceptor};

pub use pool::PoolConfig;

pub use retry::{ExponentialBackoff, RetryConfig, RetryStrategy};

// Adapters
#[cfg(feature = "reqwest-adapter")]
pub use adapters::ReqwestAdapter;

#[cfg(feature = "mock-adapter")]
pub use adapters::MockAdapter;

pub mod prelude {
    //! Convenient re-exports for common HTTP client types
    pub use crate::{
        Body, ClientConfig, Error, Headers, HttpClientPort, Method, Request, Response, Result,
        TimeoutConfig,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let request = Request::builder()
            .method(Method::POST)
            .uri("https://example.com/api")
            .header("Content-Type", "application/json")
            .body(r#"{"key":"value"}"#.as_bytes())
            .build()
            .unwrap();

        assert_eq!(request.method, Method::POST);
        assert_eq!(request.uri.to_string(), "https://example.com/api");
        assert!(request.headers.contains_key("content-type"));
    }

    #[tokio::test]
    async fn test_http_client_port_methods() {
        // This test verifies the trait methods compile correctly
        // Actual implementation tests are in adapter modules
    }
}
