// Tauri command definitions (public API)
// This module will contain all Tauri commands that form the IPC interface
// between the Rust backend and the frontend

use crate::types::{CommandError, ProviderId};
use crate::providers::Provider;
use crate::state::AppState;
use tauri::State;
use log::{info, error};

/// Gets all available providers
/// Returns the list of all 3 providers with their current state
#[tauri::command]
pub fn get_providers(state: State<AppState>) -> Result<Vec<Provider>, CommandError> {
    info!("Command: get_providers called");

    let manager = state.provider_manager.lock().map_err(|e| {
        error!("Failed to acquire lock on provider_manager: {}", e);
        CommandError::internal("Failed to access provider state")
    })?;

    let providers = manager.get_all_providers().to_vec();
    info!("Returning {} providers", providers.len());

    Ok(providers)
}

/// Updates the selection state of a provider
/// Validates that at least one provider remains selected (FR-004)
/// Validates that maximum 3 providers are selected (TC-005)
#[tauri::command]
pub fn update_provider_selection(
    state: State<AppState>,
    provider_id: ProviderId,
    is_selected: bool,
) -> Result<Provider, CommandError> {
    info!(
        "Command: update_provider_selection called for {:?}, is_selected: {}",
        provider_id, is_selected
    );

    let mut manager = state.provider_manager.lock().map_err(|e| {
        error!("Failed to acquire lock on provider_manager: {}", e);
        CommandError::internal("Failed to access provider state")
    })?;

    let provider = manager.update_provider_selection(provider_id, is_selected)?;

    info!(
        "Provider {:?} selection updated successfully to: {}",
        provider_id, is_selected
    );

    Ok(provider)
}

// Future commands to be implemented:
// - get_layout_configuration (US1)
// - create_provider_webview (US4)
// - submit_prompt (US1)
// - get_submission_status (US3)
// - check_authentication (US4)
