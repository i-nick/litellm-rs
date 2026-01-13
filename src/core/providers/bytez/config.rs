//! Bytez Configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::core::providers::base::BaseConfig;
use crate::core::traits::provider::ProviderConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytezConfig {
    #[serde(flatten)]
    pub base: BaseConfig,
}

impl Default for BytezConfig {
    fn default() -> Self {
        Self {
            base: BaseConfig {
                api_key: None,
                api_base: Some(super::DEFAULT_BASE_URL.to_string()),
                timeout: 60,
                max_retries: 3,
                headers: HashMap::new(),
                organization: None,
                api_version: None,
            },
        }
    }
}

impl BytezConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        let mut config = Self::default();
        config.base.api_key = Some(api_key.into());
        config
    }

    pub fn from_env() -> Result<Self, String> {
        let api_key = std::env::var("BYTEZ_API_KEY")
            .map_err(|_| "BYTEZ_API_KEY environment variable is required")?;
        Ok(Self::new(api_key))
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.base.api_key = Some(api_key.into());
        self
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base.api_base = Some(base_url.into());
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.base.timeout = timeout;
        self
    }
}

impl ProviderConfig for BytezConfig {
    fn validate(&self) -> Result<(), String> {
        self.base.validate("bytez")?;
        Ok(())
    }

    fn api_key(&self) -> Option<&str> {
        self.base.api_key.as_deref()
    }

    fn api_base(&self) -> Option<&str> {
        self.base.api_base.as_deref()
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(self.base.timeout)
    }

    fn max_retries(&self) -> u32 {
        self.base.max_retries
    }
}
