#!/bin/bash

# Script to create provider template files
# Usage: ./create_providers.sh

PROVIDERS=("empower" "exa_ai" "firecrawl")

for PROVIDER in "${PROVIDERS[@]}"; do
    PROVIDER_DIR="src/core/providers/${PROVIDER}"
    mkdir -p "${PROVIDER_DIR}"

    # Capitalize provider name for struct names
    PROVIDER_STRUCT=$(echo "${PROVIDER}" | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2))}1' | sed 's/ //g')

    # mod.rs
    cat > "${PROVIDER_DIR}/mod.rs" <<EOF
//! ${PROVIDER_STRUCT} Provider

pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod provider;
pub mod streaming;

pub use client::${PROVIDER_STRUCT}Client;
pub use config::${PROVIDER_STRUCT}Config;
pub use error::${PROVIDER_STRUCT}ErrorMapper;
pub use models::{${PROVIDER_STRUCT}ModelRegistry, get_${PROVIDER}_registry};
pub use provider::${PROVIDER_STRUCT}Provider;
EOF

    # config.rs
    cat > "${PROVIDER_DIR}/config.rs" <<EOF
//! ${PROVIDER_STRUCT} Configuration

use crate::core::traits::ProviderConfig;
use crate::define_provider_config;

define_provider_config!(${PROVIDER_STRUCT}Config {});

impl ${PROVIDER_STRUCT}Config {
    pub fn from_env() -> Self {
        Self::new("${PROVIDER}")
    }

    pub fn get_api_base(&self) -> String {
        self.base
            .api_base
            .clone()
            .unwrap_or_else(|| "https://api.example.com".to_string())
    }
}

impl ProviderConfig for ${PROVIDER_STRUCT}Config {
    fn validate(&self) -> Result<(), String> {
        self.base.validate("${PROVIDER}")
    }

    fn api_key(&self) -> Option<&str> {
        self.base.api_key.as_deref()
    }

    fn api_base(&self) -> Option<&str> {
        self.base.api_base.as_deref()
    }

    fn timeout(&self) -> std::time::Duration {
        self.base.timeout_duration()
    }

    fn max_retries(&self) -> u32 {
        self.base.max_retries
    }
}
EOF

    echo "Created provider: ${PROVIDER}"
done

echo "All providers created successfully!"
