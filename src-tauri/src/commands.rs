// Tauri command definitions (public API)
// This module will contain all Tauri commands that form the IPC interface
// between the Rust backend and the frontend

use crate::layout::{calculator, LayoutConfiguration};
use crate::providers::Provider;
use crate::state::AppState;
use crate::types::{CommandError, ProviderId};
use crate::{log_error, log_info};
use log::{error, info};
use tauri::State;

/// Gets all available providers
/// Returns the list of all 3 providers with their current state
#[tauri::command]
pub fn get_providers(state: State<AppState>) -> Result<Vec<Provider>, CommandError> {
    log_info!("Command: get_providers called", {
        "command": "get_providers"
    });

    let manager = state.provider_manager.lock().map_err(|e| {
        log_error!("Failed to acquire lock on provider_manager", {
            "command": "get_providers",
            "error": e.to_string()
        });
        CommandError::internal("Failed to access provider state")
    })?;

    let providers = manager.get_all_providers().to_vec();
    log_info!("Returning providers", {
        "command": "get_providers",
        "provider_count": providers.len()
    });

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
    log_info!("Command: update_provider_selection called", {
        "command": "update_provider_selection",
        "provider_id": format!("{:?}", provider_id),
        "is_selected": is_selected
    });

    let mut manager = state.provider_manager.lock().map_err(|e| {
        log_error!("Failed to acquire lock on provider_manager", {
            "command": "update_provider_selection",
            "error": e.to_string()
        });
        CommandError::internal("Failed to access provider state")
    })?;

    let provider = manager.update_provider_selection(provider_id, is_selected)?;

    log_info!("Provider selection updated successfully", {
        "command": "update_provider_selection",
        "provider_id": format!("{:?}", provider_id),
        "is_selected": is_selected
    });

    Ok(provider)
}

/// Gets the layout configuration based on currently selected providers
/// Calculates split-screen panel dimensions (1=full, 2=split, 3=grid)
#[tauri::command]
pub fn get_layout_configuration(
    state: State<AppState>,
) -> Result<LayoutConfiguration, CommandError> {
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

    log_info!("Command: submit_prompt called", {
        "command": "submit_prompt",
        "prompt_length": prompt.len()
    });

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
        return Err(CommandError::validation(
            "At least one provider must be selected",
        ));
    }

    info!(
        "Submitting prompt to {} selected providers",
        selected_providers.len()
    );

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
        let submission = state
            .status_tracker
            .create_submission(provider.id, prompt.clone())?;

        info!(
            "Created submission {} for provider {:?}",
            submission.id, provider.id
        );

        // Get provider config for selectors
        let config = provider_configs.get_config(provider.id)?;

        // T112: Generate injection script
        let script =
            injector.prepare_injection(&config.input_selectors, &config.submit_selectors, &prompt);

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
    info!(
        "Command: get_submission_status called for {}",
        submission_id
    );

    let submission = state.status_tracker.get_status(&submission_id)?;

    info!("Retrieved submission status: {:?}", submission.status);

    Ok(submission)
}

