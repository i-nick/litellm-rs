//! Build and version information
//!
//! This module exposes compile-time version information including:
//! - Package version from Cargo.toml
//! - Git commit hash
//! - Build timestamp
//! - Rust compiler version

/// Package version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Git commit hash (short form)
pub const GIT_HASH: &str = env!("GIT_HASH");

/// Build timestamp (Unix epoch seconds)
pub const BUILD_TIME: &str = env!("BUILD_TIME");

/// Rust compiler version used for build
pub const RUST_VERSION: &str = env!("RUST_VERSION");

/// Full version string with git hash
pub fn full_version() -> String {
    format!("{}-{}", VERSION, GIT_HASH)
}

/// Detailed build information
pub fn build_info() -> BuildInfo {
    BuildInfo {
        version: VERSION,
        git_hash: GIT_HASH,
        build_time: BUILD_TIME,
        rust_version: RUST_VERSION,
    }
}

/// Build information structure
#[derive(Debug, Clone, serde::Serialize)]
pub struct BuildInfo {
    pub version: &'static str,
    pub git_hash: &'static str,
    pub build_time: &'static str,
    pub rust_version: &'static str,
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{} (built {} with {})",
            self.version, self.git_hash, self.build_time, self.rust_version
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_git_hash_not_empty() {
        assert!(!GIT_HASH.is_empty());
    }

    #[test]
    fn test_full_version_format() {
        let full = full_version();
        assert!(full.contains('-'));
        assert!(full.starts_with(VERSION));
    }

    #[test]
    fn test_build_info_serializable() {
        let info = build_info();
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("version"));
        assert!(json.contains("git_hash"));
    }
}
