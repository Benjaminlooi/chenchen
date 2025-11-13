use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::types::ProviderId;

/// Represents a webview session configuration for a provider
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebviewSession {
    pub provider_id: ProviderId,
    #[cfg(not(target_os = "macos"))]
    pub data_directory: String,
    #[cfg(target_os = "macos")]
    pub data_store_identifier: String,
    pub is_persistent: bool,
    pub last_activity: String, // ISO 8601 timestamp
}

/// Information about a created webview
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WebviewInfo {
    pub provider_id: ProviderId,
    pub label: String,
    pub url: String,
    pub is_persistent: bool,
    #[cfg(not(target_os = "macos"))]
    pub data_path: String,
    #[cfg(target_os = "macos")]
    pub data_store_id: String,
}

pub mod manager;
