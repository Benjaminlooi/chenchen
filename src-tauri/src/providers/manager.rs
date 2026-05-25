// Provider selection and management logic

use super::config::ProviderConfigs;
use super::Provider;
use crate::types::{CommandError, ProviderId};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderPreferences {
    selected_providers: Vec<ProviderId>,
}

/// Manages the three LLM providers and their selection state
pub struct ProviderManager {
    providers: Vec<Provider>,
    preferences_path: Option<PathBuf>,
}

impl ProviderManager {
    /// Creates a new ProviderManager with all three providers initialized
    /// Loads is_selected defaults from providers.json config
    pub fn new() -> Self {
        Self::with_optional_preferences_path(None)
    }

    pub fn with_preferences_path(preferences_path: PathBuf) -> Self {
        Self::with_optional_preferences_path(Some(preferences_path))
    }

    fn with_optional_preferences_path(preferences_path: Option<PathBuf>) -> Self {
        // Load provider configs to get is_selected defaults
        let configs = ProviderConfigs::load().unwrap_or_else(|e| {
            eprintln!("Warning: Failed to load provider configs: {}. Using default is_selected=true for all providers.", e);
            // Return a default that will cause fallback behavior
            ProviderConfigs {
                version: "1.0.0".to_string(),
                providers: std::collections::HashMap::new(),
            }
        });

        // Helper to get is_selected from config or default to true
        let get_is_selected = |provider_id: ProviderId| -> bool {
            configs
                .get_config(provider_id)
                .map(|config| config.is_selected)
                .unwrap_or(true) // Default to selected if config not found
        };

        let mut providers = vec![
            Provider::new(ProviderId::ChatGPT, get_is_selected(ProviderId::ChatGPT)),
            Provider::new(ProviderId::Gemini, get_is_selected(ProviderId::Gemini)),
            Provider::new(ProviderId::Claude, get_is_selected(ProviderId::Claude)),
            Provider::new(
                ProviderId::Perplexity,
                get_is_selected(ProviderId::Perplexity),
            ),
            Provider::new(ProviderId::DeepSeek, get_is_selected(ProviderId::DeepSeek)),
            Provider::new(ProviderId::Ollama, get_is_selected(ProviderId::Ollama)),
        ];

        if let Some(path) = &preferences_path {
            Self::apply_saved_preferences(&mut providers, path);
        }

        Self {
            providers,
            preferences_path,
        }
    }

    fn apply_saved_preferences(providers: &mut [Provider], preferences_path: &Path) {
        let Ok(contents) = fs::read_to_string(preferences_path) else {
            return;
        };

        let Ok(preferences) = serde_json::from_str::<ProviderPreferences>(&contents) else {
            return;
        };

        if preferences.selected_providers.is_empty() || preferences.selected_providers.len() > 3 {
            return;
        }

        if preferences
            .selected_providers
            .iter()
            .any(|provider_id| !providers.iter().any(|provider| provider.id == *provider_id))
        {
            return;
        }

        for provider in providers {
            provider.is_selected = preferences.selected_providers.contains(&provider.id);
        }
    }

    fn persist_preferences(&self) -> Result<(), CommandError> {
        let Some(path) = &self.preferences_path else {
            return Ok(());
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                CommandError::internal(format!("Failed to create preferences directory: {}", e))
            })?;
        }

        let preferences = ProviderPreferences {
            selected_providers: self
                .providers
                .iter()
                .filter(|provider| provider.is_selected)
                .map(|provider| provider.id)
                .collect(),
        };

        let contents = serde_json::to_string(&preferences).map_err(|e| {
            CommandError::internal(format!("Failed to serialize provider preferences: {}", e))
        })?;

        fs::write(path, contents).map_err(|e| {
            CommandError::internal(format!("Failed to write provider preferences: {}", e))
        })
    }

    /// Returns all providers
    pub fn get_all_providers(&self) -> &[Provider] {
        &self.providers
    }

    /// Updates the selection state of a provider
    /// Returns an error if attempting to deselect the last selected provider
    pub fn update_provider_selection(
        &mut self,
        provider_id: ProviderId,
        is_selected: bool,
    ) -> Result<Provider, CommandError> {
        // Validation: Cannot deselect last provider (FR-004)
        if !is_selected {
            let selected_count = self.selected_count();
            if selected_count == 1 {
                return Err(CommandError::validation(
                    "At least one provider must be selected",
                ));
            }
        }

        // Validation: Cannot select more than 3 providers (TC-005)
        if is_selected {
            let selected_count = self.selected_count();
            if selected_count >= 3 {
                return Err(CommandError::validation(
                    "Maximum 3 providers can be selected",
                ));
            }
        }

        // Find and update the provider
        let provider = self
            .providers
            .iter_mut()
            .find(|p| p.id == provider_id)
            .ok_or_else(|| {
                CommandError::not_found(format!("Provider {:?} not found", provider_id))
            })?;

        provider.is_selected = is_selected;
        let provider = provider.clone();
        self.persist_preferences()?;
        Ok(provider)
    }

    /// Returns the number of currently selected providers
    fn selected_count(&self) -> usize {
        self.providers.iter().filter(|p| p.is_selected).count()
    }

    /// Returns only the selected providers
    pub fn get_selected_providers(&self) -> Vec<&Provider> {
        self.providers.iter().filter(|p| p.is_selected).collect()
    }
}

impl Default for ProviderManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_six_providers() {
        let manager = ProviderManager::new();
        let providers = manager.get_all_providers();

        assert_eq!(providers.len(), 6);
        assert!(providers.iter().any(|p| p.id == ProviderId::ChatGPT));
        assert!(providers.iter().any(|p| p.id == ProviderId::Gemini));
        assert!(providers.iter().any(|p| p.id == ProviderId::Claude));
    }

    #[test]
    fn test_cannot_deselect_last_provider() {
        let mut manager = ProviderManager::new();

        // ChatGPT and Gemini start selected by default.
        manager
            .update_provider_selection(ProviderId::ChatGPT, false)
            .expect("Should allow deselecting first provider");

        // Now try to deselect the last one - should fail
        let result = manager.update_provider_selection(ProviderId::Gemini, false);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code, "ValidationError");
    }

    #[test]
    fn test_can_select_all_three_providers() {
        let mut manager = ProviderManager::new();

        // ChatGPT and Gemini start selected by default; Claude can be added.
        let selected_count = manager.get_selected_providers().len();
        assert_eq!(selected_count, 2);

        manager
            .update_provider_selection(ProviderId::Claude, true)
            .expect("Should allow selecting a third provider");

        let selected_count = manager.get_selected_providers().len();
        assert_eq!(selected_count, 3);
    }

    #[test]
    fn test_can_toggle_selection() {
        let mut manager = ProviderManager::new();

        // ChatGPT and Gemini start selected by default.
        assert_eq!(manager.get_selected_providers().len(), 2);

        // Deselect one provider
        let result = manager.update_provider_selection(ProviderId::ChatGPT, false);
        assert!(result.is_ok());

        let selected = manager.get_selected_providers();
        assert_eq!(selected.len(), 1);
        assert!(selected.iter().any(|p| p.id == ProviderId::Gemini));

        // Re-select it
        let result = manager.update_provider_selection(ProviderId::ChatGPT, true);
        assert!(result.is_ok());

        let selected = manager.get_selected_providers();
        assert_eq!(selected.len(), 2);
    }
}
