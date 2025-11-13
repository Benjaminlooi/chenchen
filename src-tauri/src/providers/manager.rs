// Provider selection and management logic

use super::Provider;
use crate::types::{CommandError, ProviderId};

/// Manages the three LLM providers and their selection state
pub struct ProviderManager {
    providers: Vec<Provider>,
}

impl ProviderManager {
    /// Creates a new ProviderManager with all three providers initialized
    pub fn new() -> Self {
        Self {
            providers: vec![
                Provider::new(ProviderId::ChatGPT),
                Provider::new(ProviderId::Gemini),
                Provider::new(ProviderId::Claude),
            ],
        }
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
        Ok(provider.clone())
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
    fn test_new_returns_three_providers() {
        let manager = ProviderManager::new();
        let providers = manager.get_all_providers();

        assert_eq!(providers.len(), 3);
        assert!(providers.iter().any(|p| p.id == ProviderId::ChatGPT));
        assert!(providers.iter().any(|p| p.id == ProviderId::Gemini));
        assert!(providers.iter().any(|p| p.id == ProviderId::Claude));
    }

    #[test]
    fn test_cannot_deselect_last_provider() {
        let mut manager = ProviderManager::new();

        manager
            .update_provider_selection(ProviderId::ChatGPT, true)
            .expect("Should allow selecting first provider");

        let result = manager.update_provider_selection(ProviderId::ChatGPT, false);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code, "ValidationError");
    }

    #[test]
    fn test_can_select_all_three_providers() {
        let mut manager = ProviderManager::new();

        manager
            .update_provider_selection(ProviderId::ChatGPT, true)
            .expect("Should allow selecting first provider");
        manager
            .update_provider_selection(ProviderId::Gemini, true)
            .expect("Should allow selecting second provider");
        manager
            .update_provider_selection(ProviderId::Claude, true)
            .expect("Should allow selecting third provider");

        let selected_count = manager.get_selected_providers().len();
        assert_eq!(selected_count, 3);
    }

    #[test]
    fn test_can_toggle_selection() {
        let mut manager = ProviderManager::new();

        manager
            .update_provider_selection(ProviderId::ChatGPT, true)
            .unwrap();
        manager
            .update_provider_selection(ProviderId::Gemini, true)
            .unwrap();

        let result = manager.update_provider_selection(ProviderId::ChatGPT, false);
        assert!(result.is_ok());

        let selected = manager.get_selected_providers();
        assert_eq!(selected.len(), 1);
        assert!(selected.iter().any(|p| p.id == ProviderId::Gemini));
    }
}
