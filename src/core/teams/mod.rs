//! Team Management Module
//!
//! This module provides comprehensive team management functionality including:
//! - Team CRUD operations
//! - Member management (add, remove, update roles)
//! - Team settings and configuration
//! - Usage statistics tracking
//!
//! ## Architecture
//!
//! The module follows a repository pattern with:
//! - `TeamRepository` trait for storage abstraction
//! - `InMemoryTeamRepository` for testing and development
//! - `PostgresTeamRepository` for production (requires `postgres` feature)
//! - `TeamManager` for business logic coordination
//!
//! ## Usage
//!
//! ```ignore
//! use litellm_rs::core::teams::{TeamManager, InMemoryTeamRepository, CreateTeamRequest};
//! use std::sync::Arc;
//!
//! let repo = Arc::new(InMemoryTeamRepository::new());
//! let manager = TeamManager::new(repo);
//!
//! let request = CreateTeamRequest {
//!     name: "my-team".to_string(),
//!     display_name: Some("My Team".to_string()),
//!     description: None,
//!     settings: None,
//! };
//!
//! let team = manager.create_team(request).await?;
//! ```

pub mod manager;
pub mod repository;

// Re-export commonly used types
pub use manager::{
    AddMemberRequest, CreateTeamRequest, TeamManager, TeamUsageStats, UpdateRoleRequest,
    UpdateTeamRequest,
};
pub use repository::{InMemoryTeamRepository, TeamRepository};

#[cfg(feature = "postgres")]
pub use repository::postgres::PostgresTeamRepository;

// Re-export team model types for convenience
pub use crate::core::models::team::{
    MemberStatus, Team, TeamMember, TeamRole, TeamSettings, TeamStatus, TeamVisibility,
};
