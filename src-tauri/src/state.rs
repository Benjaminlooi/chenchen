use std::sync::Mutex;
use crate::providers::manager::ProviderManager;

/// Application state shared across Tauri commands
/// This state is managed by Tauri and accessible to all commands
pub struct AppState {
    /// Provider manager for handling provider selection and configuration
    pub provider_manager: Mutex<ProviderManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            provider_manager: Mutex::new(ProviderManager::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
