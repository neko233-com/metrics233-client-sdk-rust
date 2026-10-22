//! Rust client entry point for metrics233-server.
//!
//! This package re-exports the verified `metrics233` transport rather than
//! introducing another queue, retry policy, HTTP stack or TLS implementation.
//! Construction is inert; call `start`, `flush` and `shutdown` explicitly.
//!
//! ```no_run
//! use metrics233_client_sdk_rust::{Client, Identity};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let metrics = Client::builder(std::env::var("METRICS233_URL")?)
//!     .with_token(std::env::var("METRICSPANEL_INGEST_TOKEN")?)
//!     .with_identity(Identity::new("1").with_service("game-api")
//!         .with_role("logic").with_environment("test").with_instance("node-1"))
//!     .build()?;
//! metrics.counter("game_requests_total", 1.0)?;
//! metrics.gauge("game_players_online", 42.0)?;
//! metrics.shutdown().await?;
//! # Ok(()) }
//! ```
pub use metrics233::{Batch, Builder, Client, Error, Identity, Labels, Limits, Sample, Stats};
