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

/// Submits a prompt to all selected providers
/// Creates submissions, generates injection scripts, and tracks status
///
/// Returns the created submissions with their IDs for status tracking
#[tauri::command]
pub fn submit_prompt(
    state: State<AppState>,
    prompt: String,
) -> Result<Vec<crate::status::Submission>, CommandError> {
    use crate::injection::injector::Injector;

    info!("Command: submit_prompt called with prompt length: {}", prompt.len());

    // T109: Validate non-empty prompt
    if prompt.trim().is_empty() {
        return Err(CommandError::validation("Prompt cannot be empty"));
    }

    // Get selected providers
    let manager = state.provider_manager.lock().map_err(|e| {
        error!("Failed to acquire lock on provider_manager: {}", e);
        CommandError::internal("Failed to access provider state")
    })?;

    let selected_providers = manager.get_selected_providers();

    // T109: Validate at least 1 provider selected
    if selected_providers.is_empty() {
        return Err(CommandError::validation("At least one provider must be selected"));
    }

    info!("Submitting prompt to {} selected providers", selected_providers.len());

    // Get provider configs
    let provider_configs = state.provider_configs.as_ref().ok_or_else(|| {
        error!("Provider configurations not loaded");
        CommandError::internal("Provider configurations not available")
    })?;

    // Create submissions for each selected provider (T110)
    let mut submissions = Vec::new();
    let injector = Injector::new().map_err(|e| {
        error!("Failed to create Injector: {}", e);
        CommandError::internal("Failed to initialize injector")
    })?;

    for provider in selected_providers {
        // Create submission entity
        let submission = state.status_tracker.create_submission(
            provider.id,
            prompt.clone(),
        )?;

        info!("Created submission {} for provider {:?}", submission.id, provider.id);

        // Get provider config for selectors
        let config = provider_configs.get_config(provider.id)?;

        // T112: Generate injection script
        let script = injector.prepare_injection(
            &config.input_selectors,
            &config.submit_selectors,
            &prompt,
        );

        info!(
            "Generated injection script for provider {:?} ({} chars)",
            provider.id,
            script.len()
        );

        // TODO T111: Spawn async task for each submission (concurrent execution)
        // TODO T112: Call Injector::execute() for each provider in async task
        // TODO T113: Update Submission status based on injection result
        // TODO T114: Emit submission_status_changed event after each status update
        //
        // For now, submissions remain in Pending state
        // Actual webview.eval() execution will be implemented when webview handles are available

        submissions.push(submission);
    }

    info!("Created {} submissions", submissions.len());

    Ok(submissions)
}

/// Gets the status of a specific submission
#[tauri::command]
pub fn get_submission_status(
    state: State<AppState>,
    submission_id: String,
) -> Result<crate::status::Submission, CommandError> {
    info!("Command: get_submission_status called for {}", submission_id);

    let submission = state.status_tracker.get_status(&submission_id)?;

    info!("Retrieved submission status: {:?}", submission.status);

    Ok(submission)
}

/// Checks if a provider webview is authenticated
/// Returns authentication status including whether login is required
#[tauri::command]
pub fn check_authentication(
    state: State<AppState>,
    provider_id: ProviderId,
) -> Result<crate::webview::AuthenticationStatus, CommandError> {
    use crate::webview::manager::WebviewManager;
    use std::path::PathBuf;

    info!("Command: check_authentication called for {:?}", provider_id);

    // Get provider configs to access auth_check_selectors
    let provider_configs = state.provider_configs.as_ref().ok_or_else(|| {
        error!("Provider configurations not loaded");
        CommandError::internal("Provider configurations not available")
    })?;

    let config = provider_configs.get_config(provider_id)?;

    // Create webview manager (we don't need the actual data directory for auth check script generation)
    let manager = WebviewManager::new(PathBuf::from("/tmp")).map_err(|e| {
        error!("Failed to create WebviewManager: {}", e);
        CommandError::internal("Failed to create webview manager")
    })?;

    // Generate auth check script
    let _auth_script = manager.generate_auth_check_script(&config.auth_check_selectors);

    // TODO: Execute auth_script in actual webview and parse result
    // For now, return a mock status (assumes authenticated for demo purposes)
    let auth_status = manager.create_auth_status_mock(provider_id, true);

    info!(
        "Auth check for {:?}: authenticated={}, requires_login={}",
        provider_id, auth_status.is_authenticated, auth_status.requires_login
    );

    Ok(auth_status)
}
