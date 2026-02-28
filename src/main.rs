//! LiteLLM-RS - High-performance async AI gateway
//!
//! Async gateway service supporting multiple AI providers

#![allow(missing_docs)]

use litellm_rs::server;
use std::process::ExitCode;
#[cfg(feature = "tracing")]
use tracing::Level;

fn init_logging() {
    #[cfg(feature = "tracing")]
    {
        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_target(false)
            .with_thread_ids(false)
            .init();
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    init_logging();

    // Start server (auto-loads config/gateway.yaml)
    match server::builder::run_server().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            // Print error using Display (not Debug) to preserve newlines
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
