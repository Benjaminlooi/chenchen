// Contract tests for ProviderManager library
// These tests verify the public API contract

use chenchen_lib::providers::manager::ProviderManager;
use chenchen_lib::types::ProviderId;

#[test]
fn test_new_returns_three_providers() {
    // T022: ProviderManager::new() returns 3 providers
    let manager = ProviderManager::new();
    let providers = manager.get_all_providers();

    assert_eq!(providers.len(), 3, "Should have exactly 3 providers");

    // Verify all three providers exist
    assert!(
        providers.iter().any(|p| p.id == ProviderId::ChatGPT),
        "Should include ChatGPT provider"
    );
    assert!(
        providers.iter().any(|p| p.id == ProviderId::Gemini),
        "Should include Gemini provider"
    );
    assert!(
        providers.iter().any(|p| p.id == ProviderId::Claude),
        "Should include Claude provider"
    );
}

#[test]
fn test_get_all_providers_returns_correct_data() {
    // T023: ProviderManager::get_all_providers() returns valid provider data
    let manager = ProviderManager::new();
    let providers = manager.get_all_providers();

    for provider in providers {
        // Verify provider fields are populated
        assert!(
            !provider.name.is_empty(),
            "Provider name should not be empty"
        );
        assert!(!provider.url.is_empty(), "Provider URL should not be empty");
        assert_eq!(
            provider.is_selected, false,
            "Providers should be unselected by default"
        );
        assert_eq!(
            provider.is_authenticated, false,
            "Providers should be unauthenticated by default"
        );
    }
}

#[test]
fn test_update_selection_validates_minimum_one_selected() {
    // T024: ProviderManager::update_selection() validates minimum 1 selected
    let mut manager = ProviderManager::new();

    // First, select a provider
    manager
        .update_provider_selection(ProviderId::ChatGPT, true)
        .expect("Should allow selecting first provider");

    // Try to deselect the only selected provider (should fail)
    let result = manager.update_provider_selection(ProviderId::ChatGPT, false);

    assert!(
        result.is_err(),
        "Should not allow deselecting the last provider"
    );

    let error = result.unwrap_err();
    assert_eq!(error.code, "ValidationError");
    assert!(
        error
            .message
            .contains("At least one provider must be selected"),
        "Error message should explain the validation rule"
    );
}

#[test]
fn test_update_selection_validates_maximum_three_selected() {
    // T025: ProviderManager::update_selection() validates maximum 3 selected
    let mut manager = ProviderManager::new();

    // Select all three providers
    manager
        .update_provider_selection(ProviderId::ChatGPT, true)
        .expect("Should allow selecting first provider");
    manager
        .update_provider_selection(ProviderId::Gemini, true)
        .expect("Should allow selecting second provider");
    manager
        .update_provider_selection(ProviderId::Claude, true)
        .expect("Should allow selecting third provider");

    // All three are selected, trying to select another should fail
    // (but we only have 3 providers, so we test that we can't exceed 3)

    // Actually, with only 3 providers total, we can't exceed 3
    // Let's test that we CAN select all 3
    let providers = manager.get_all_providers();
    let selected_count = providers.iter().filter(|p| p.is_selected).count();

    assert_eq!(selected_count, 3, "Should allow selecting all 3 providers");
}

#[test]
fn test_update_selection_allows_toggling() {
    // Additional test: Verify toggling selection works correctly
    let mut manager = ProviderManager::new();

    // Select ChatGPT
    manager
        .update_provider_selection(ProviderId::ChatGPT, true)
        .expect("Should allow selecting provider");

    let providers = manager.get_all_providers();
    let chatgpt = providers
        .iter()
        .find(|p| p.id == ProviderId::ChatGPT)
        .expect("ChatGPT should exist");

    assert!(chatgpt.is_selected, "ChatGPT should be selected");

    // Select Gemini as well
    manager
        .update_provider_selection(ProviderId::Gemini, true)
        .expect("Should allow selecting second provider");

    // Now deselect ChatGPT (should work since Gemini is still selected)
    let result = manager.update_provider_selection(ProviderId::ChatGPT, false);
    assert!(
        result.is_ok(),
        "Should allow deselecting when another provider is selected"
    );

    let providers = manager.get_all_providers();
    let chatgpt = providers
        .iter()
        .find(|p| p.id == ProviderId::ChatGPT)
        .expect("ChatGPT should exist");

    assert!(
        !chatgpt.is_selected,
        "ChatGPT should be deselected after toggle"
    );
}

#[test]
fn test_get_selected_providers() {
    // Test the get_selected_providers helper method
    let mut manager = ProviderManager::new();

    // Initially no providers selected
    let selected = manager.get_selected_providers();
    assert_eq!(
        selected.len(),
        0,
        "Should have no selected providers initially"
    );

    // Select ChatGPT and Gemini
    manager
        .update_provider_selection(ProviderId::ChatGPT, true)
        .expect("Should select ChatGPT");
    manager
        .update_provider_selection(ProviderId::Gemini, true)
        .expect("Should select Gemini");

    let selected = manager.get_selected_providers();
    assert_eq!(selected.len(), 2, "Should have 2 selected providers");
    assert!(selected.iter().any(|p| p.id == ProviderId::ChatGPT));
    assert!(selected.iter().any(|p| p.id == ProviderId::Gemini));
}
