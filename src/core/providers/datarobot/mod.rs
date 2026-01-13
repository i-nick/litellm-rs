//! DataRobot Provider
//!
//! DataRobot AI platform provider for enterprise ML and LLM deployments.

pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod provider;
pub mod streaming;

pub use client::DataRobotClient;
pub use config::DataRobotConfig;
pub use error::DataRobotErrorMapper;
pub use models::{DataRobotModelRegistry, get_datarobot_registry};
pub use provider::DataRobotProvider;
