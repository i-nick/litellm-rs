//! Runway ML Provider
//!
//! Provides integration with Runway ML's video and image generation APIs.
//! Supports Gen-3 Alpha, Gen-3 Alpha Turbo, and image-to-video generation.
//!
//! # Features
//!
//! - **Text-to-Video**: Generate videos from text prompts using Gen-3 models
//! - **Image-to-Video**: Animate still images into videos
//! - **Async Generation**: Submit tasks and poll for completion
//!
//! # Example
//!
//! ```rust,no_run
//! use litellm_rs::core::providers::runwayml::{RunwayMLProvider, RunwayMLConfig};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create provider with API key
//!     let config = RunwayMLConfig::new("your-api-key");
//!     let provider = RunwayMLProvider::new(config)?;
//!
//!     // Generate a video
//!     let video = provider.generate_video(
//!         Some("A cat playing piano in a jazz club".to_string()),
//!         None,            // No input image
//!         Some("gen3a_turbo"),
//!         Some(5),         // 5 seconds
//!         Some("16:9".to_string()),
//!         None,            // No seed
//!     ).await?;
//!
//!     println!("Video URLs: {:?}", video.video_urls);
//!     Ok(())
//! }
//! ```
//!
//! # Configuration
//!
//! The provider can be configured via environment variables:
//!
//! - `RUNWAYML_API_KEY`: Required. Your Runway ML API key.
//! - `RUNWAYML_API_BASE`: Optional. Override the API base URL.
//! - `RUNWAYML_POLLING_DELAY`: Optional. Seconds between status polls (default: 2).
//! - `RUNWAYML_POLLING_RETRIES`: Optional. Max polling retries (default: 300).
//! - `RUNWAYML_VIDEO_DURATION`: Optional. Default video duration in seconds (5 or 10).
//! - `RUNWAYML_VIDEO_RESOLUTION`: Optional. Default resolution (720p or 1080p).
//! - `RUNWAYML_WATERMARK`: Optional. Whether to add watermark (true/false).

pub mod config;
pub mod error;
pub mod models;
pub mod provider;

pub use config::RunwayMLConfig;
pub use error::RunwayMLErrorMapper;
pub use models::{RunwayMLModelRegistry, RunwayModelType, RunwayTaskType, get_runwayml_registry};
pub use provider::{
    CreateTaskRequest, RunwayMLProvider, TaskResponse, TaskStatus, VideoGenerationResponse,
};
