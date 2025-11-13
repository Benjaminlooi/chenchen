// Provider management library
// Handles provider selection, configuration, and state

pub mod manager;
pub mod config;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::types::ProviderId;

/// Represents an LLM provider with its configuration and state
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub url: String,
    pub is_selected: bool,
    pub is_authenticated: bool,
    pub selector_config_id: String,
}

impl Provider {
    pub fn new(id: ProviderId) -> Self {
        Self {
            name: id.as_str().to_string(),
            url: id.url().to_string(),
            id,
            is_selected: false,
            is_authenticated: false,
            selector_config_id: id.as_str().to_string(),
        }
    }
}
