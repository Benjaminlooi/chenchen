// Provider selector configuration loading and management

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::types::{CommandError, ProviderId};
use std::collections::HashMap;
use std::fs;
use chrono::{DateTime, Utc};

/// CSS selectors and configuration for locating elements on a provider's website
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProviderSelectorConfig {
    pub provider_id: ProviderId,
    pub version: String,
    pub input_selectors: Vec<String>,
    pub submit_selectors: Vec<String>,
    pub auth_check_selectors: Vec<String>,
    #[schemars(with = "String")]
    pub last_updated: DateTime<Utc>,
}

/// Container for all provider configurations
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProviderConfigs {
    pub version: String,
    pub providers: HashMap<String, ProviderSelectorConfig>,
}

impl ProviderConfigs {
    /// Loads provider configurations from the config/providers.json file
    pub fn load() -> Result<Self, CommandError> {
        let config_path = "config/providers.json";
        let content = fs::read_to_string(config_path).map_err(|e| {
            CommandError::internal(format!("Failed to read provider config: {}", e))
        })?;

        let configs: ProviderConfigs = serde_json::from_str(&content).map_err(|e| {
            CommandError::internal(format!("Failed to parse provider config: {}", e))
        })?;

        // Validate configurations
        configs.validate()?;

        Ok(configs)
    }

    /// Validates the loaded configurations
    fn validate(&self) -> Result<(), CommandError> {
        // Validate version format (semver)
        if !self.is_valid_semver(&self.version) {
            return Err(CommandError::validation(format!(
                "Invalid config version format: {}",
                self.version
            )));
        }

        // Validate each provider config
        for (key, config) in &self.providers {
            // Validate provider version
            if !self.is_valid_semver(&config.version) {
                return Err(CommandError::validation(format!(
                    "Invalid version format for provider {}: {}",
                    key, config.version
                )));
            }

            // Validate non-empty selector arrays
            if config.input_selectors.is_empty() {
                return Err(CommandError::validation(format!(
                    "input_selectors cannot be empty for provider {}",
                    key
                )));
            }
            if config.submit_selectors.is_empty() {
                return Err(CommandError::validation(format!(
                    "submit_selectors cannot be empty for provider {}",
                    key
                )));
            }
            if config.auth_check_selectors.is_empty() {
                return Err(CommandError::validation(format!(
                    "auth_check_selectors cannot be empty for provider {}",
                    key
                )));
            }
        }

        Ok(())
    }

    /// Simple semver validation (MAJOR.MINOR.PATCH)
    fn is_valid_semver(&self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }
        parts.iter().all(|p| p.parse::<u32>().is_ok())
    }

    /// Gets the configuration for a specific provider
    pub fn get_config(&self, provider_id: ProviderId) -> Result<&ProviderSelectorConfig, CommandError> {
        self.providers
            .get(provider_id.as_str())
            .ok_or_else(|| {
                CommandError::not_found(format!(
                    "Configuration not found for provider {:?}",
                    provider_id
                ))
            })
    }
}
