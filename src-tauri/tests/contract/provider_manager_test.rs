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
        assert!(
            provider.is_selected,
            "Providers should be selected by default"
        );
        assert!(
            !provider.is_authenticated,
            "Providers should be unauthenticated by default"
        );
    }
}

#[test]
fn test_update_selection_validates_minimum_one_selected() {
    // T024: ProviderManager::update_selection() validates minimum 1 selected
    let mut manager = ProviderManager::new();

    // All 3 providers start selected by default
    // Deselect two of them first
    manager
        .update_provider_selection(ProviderId::ChatGPT, false)
        .expect("Should allow deselecting first provider");
    manager
        .update_provider_selection(ProviderId::Gemini, false)
        .expect("Should allow deselecting second provider");

    // Try to deselect the last selected provider (should fail)
    let result = manager.update_provider_selection(ProviderId::Claude, false);

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

    // All three providers are already selected by default
    let providers = manager.get_all_providers();
    let selected_count = providers.iter().filter(|p| p.is_selected).count();
    assert_eq!(selected_count, 3, "All 3 providers should start selected");

    // Verify we can toggle and re-select without exceeding 3
    manager
        .update_provider_selection(ProviderId::ChatGPT, false)
        .expect("Should allow deselecting");

    let providers = manager.get_all_providers();
    let selected_count = providers.iter().filter(|p| p.is_selected).count();
    assert_eq!(selected_count, 2, "Should have 2 selected after deselecting one");

    // Re-select to get back to 3
    manager
        .update_provider_selection(ProviderId::ChatGPT, true)
        .expect("Should allow re-selecting to get back to 3");

    let providers = manager.get_all_providers();
    let selected_count = providers.iter().filter(|p| p.is_selected).count();
    assert_eq!(selected_count, 3, "Should have all 3 selected again");
}

#[test]
fn test_update_selection_allows_toggling() {
    // Additional test: Verify toggling selection works correctly
    let mut manager = ProviderManager::new();

    // All 3 providers start selected by default
    let providers = manager.get_all_providers();
    let chatgpt = providers
        .iter()
        .find(|p| p.id == ProviderId::ChatGPT)
        .expect("ChatGPT should exist");

    assert!(chatgpt.is_selected, "ChatGPT should be selected by default");

    // Deselect ChatGPT (should work since other providers are still selected)
    let result = manager.update_provider_selection(ProviderId::ChatGPT, false);
    assert!(
        result.is_ok(),
        "Should allow deselecting when other providers are selected"
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

    // Re-select ChatGPT
    let result = manager.update_provider_selection(ProviderId::ChatGPT, true);
    assert!(result.is_ok(), "Should allow re-selecting provider");

    let providers = manager.get_all_providers();
    let chatgpt = providers
        .iter()
        .find(|p| p.id == ProviderId::ChatGPT)
        .expect("ChatGPT should exist");

    assert!(chatgpt.is_selected, "ChatGPT should be selected again");
}

#[test]
fn test_get_selected_providers() {
    // Test the get_selected_providers helper method
    let mut manager = ProviderManager::new();

    // All 3 providers start selected by default
    let selected = manager.get_selected_providers();
    assert_eq!(
        selected.len(),
        3,
        "Should have all 3 providers selected initially"
    );

    // Deselect ChatGPT
    manager
        .update_provider_selection(ProviderId::ChatGPT, false)
        .expect("Should deselect ChatGPT");

    let selected = manager.get_selected_providers();
    assert_eq!(selected.len(), 2, "Should have 2 selected providers after deselecting one");
    assert!(selected.iter().any(|p| p.id == ProviderId::Gemini));
    assert!(selected.iter().any(|p| p.id == ProviderId::Claude));
    assert!(!selected.iter().any(|p| p.id == ProviderId::ChatGPT));
}
