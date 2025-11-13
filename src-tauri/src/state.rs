use std::sync::Mutex;
use crate::providers::manager::ProviderManager;
use crate::providers::config::ProviderConfigs;
use log::{info, warn};

/// Application state shared across Tauri commands
/// This state is managed by Tauri and accessible to all commands
pub struct AppState {
    /// Provider manager for handling provider selection and configuration
    pub provider_manager: Mutex<ProviderManager>,
    /// Provider selector configurations (CSS selectors, etc.)
    pub provider_configs: Option<ProviderConfigs>,
}

impl AppState {
    pub fn new() -> Self {
        // Try to load provider configs
        let provider_configs = match ProviderConfigs::load() {
            Ok(configs) => {
                info!("Successfully loaded provider configurations");
                Some(configs)
            }
            Err(e) => {
                warn!("Failed to load provider configurations: {}", e);
                None
            }
        };

        Self {
            provider_manager: Mutex::new(ProviderManager::new()),
            provider_configs,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
