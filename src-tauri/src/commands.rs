// Tauri command definitions (public API)
// This module will contain all Tauri commands that form the IPC interface
// between the Rust backend and the frontend

use crate::types::{CommandError, ProviderId};
use crate::providers::Provider;
use crate::layout::{LayoutConfiguration, calculator};
use crate::state::AppState;
use tauri::{State, Manager};
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

/// Gets the layout configuration based on currently selected providers
/// Calculates split-screen panel dimensions (1=full, 2=split, 3=grid)
#[tauri::command]
pub fn get_layout_configuration(state: State<AppState>) -> Result<LayoutConfiguration, CommandError> {
    info!("Command: get_layout_configuration called");

    let manager = state.provider_manager.lock().map_err(|e| {
        error!("Failed to acquire lock on provider_manager: {}", e);
        CommandError::internal("Failed to access provider state")
    })?;

    // Get only the selected providers
    let selected_providers: Vec<ProviderId> = manager
        .get_selected_providers()
        .iter()
        .map(|p| p.id)
        .collect();

    if selected_providers.is_empty() {
        return Err(CommandError::validation("No providers selected"));
    }

    // Calculate layout based on selected providers
    let layout = calculator::calculate_layout(&selected_providers);

    info!(
        "Calculated {:?} layout for {} providers",
        layout.layout_type, layout.provider_count
    );

    Ok(layout)
}

/// Creates a webview window for a provider with persistent session storage
/// Returns information about the created webview including session configuration
#[tauri::command]
pub fn create_provider_webview(
    app: tauri::AppHandle,
    provider_id: ProviderId,
) -> Result<crate::webview::WebviewInfo, CommandError> {
    use crate::webview::manager::WebviewManager;

    info!("Command: create_provider_webview called for {:?}", provider_id);

    // Get app data directory
    let app_data_dir = app.path().app_local_data_dir().map_err(|e| {
        error!("Failed to get app local data directory: {}", e);
        CommandError::internal("Failed to get app data directory")
    })?;

    // Create webview manager
    let manager = WebviewManager::new(app_data_dir).map_err(|e| {
        error!("Failed to create WebviewManager: {}", e);
        CommandError::internal(format!("Failed to create webview manager: {}", e))
    })?;

    // Create webview info
    let webview_info = manager.create_webview_info(provider_id);

    #[cfg(not(target_os = "macos"))]
    {
        // Ensure data directory exists
        manager.ensure_data_directory(provider_id).map_err(|e| {
            error!("Failed to create data directory: {}", e);
            CommandError::internal("Failed to create webview data directory")
        })?;
    }

    info!(
        "Created webview info for {:?}: label={}, persistent={}",
        provider_id, webview_info.label, webview_info.is_persistent
    );

    Ok(webview_info)
}

// Future commands to be implemented:
// - submit_prompt (US1)
// - get_submission_status (US3)
// - check_authentication (US4)
