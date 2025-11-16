// Tauri command definitions (public API)
// This module will contain all Tauri commands that form the IPC interface
// between the Rust backend and the frontend

use crate::layout::{calculator, LayoutConfiguration};
use crate::providers::Provider;
use crate::state::AppState;
use crate::types::{CommandError, ProviderId};
use crate::{log_error, log_info};
use log::{error, info};
use std::sync::Arc;
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
pub async fn submit_prompt(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    prompt: String,
) -> Result<Vec<crate::status::Submission>, CommandError> {
    use crate::injection::injector::Injector;
    use crate::types::SubmissionErrorType;

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

        let submission_id = submission.id.clone();
        let provider_id = provider.id;

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

        // Clone necessary data for the async task
        let app_clone = app.clone();
        let provider_url = provider.url.clone();
        let provider_name = provider.name.clone();
        let script_clone = script.clone();

        // Clone Arc references for the async task
        let status_tracker = Arc::clone(&state.status_tracker);
        let webview_manager = Arc::clone(&state.webview_manager);

        // T111: Spawn async task for concurrent execution
        tauri::async_runtime::spawn(async move {

            // Start the submission
            if let Err(e) = status_tracker.start_submission(&submission_id) {
                log_error!("Failed to start submission", {
                    "submission_id": &submission_id,
                    "provider_id": format!("{:?}", provider_id),
                    "error": e.to_string()
                });
                return;
            }

            log_info!("Starting submission execution", {
                "submission_id": &submission_id,
                "provider_id": format!("{:?}", provider_id)
            });

            // Create or get webview for this provider
            if let Err(e) = webview_manager.get_or_create_webview(
                &app_clone,
                provider_id,
                &provider_url,
                &provider_name,
            ) {
                log_error!("Failed to create webview", {
                    "submission_id": &submission_id,
                    "provider_id": format!("{:?}", provider_id),
                    "error": &e
                });
                let _ = status_tracker.fail_submission(
                    &submission_id,
                    SubmissionErrorType::NetworkError,
                    format!("Failed to create webview: {}", e),
                );
                return;
            }

            // Wait for webview to load (give it some time to navigate)
            // Use a simple timer by spawning a blocking sleep in the async runtime
            let _ = tauri::async_runtime::spawn_blocking(|| {
                std::thread::sleep(std::time::Duration::from_secs(2));
            }).await;

            // T112: Execute the injection
            let execution_result = Injector::new()
                .unwrap()
                .execute(&*webview_manager, provider_id, &script_clone)
                .await;

            // T113: Update submission status based on result
            match execution_result {
                Ok(result) if result.success => {
                    log_info!("Submission succeeded", {
                        "submission_id": &submission_id,
                        "provider_id": format!("{:?}", provider_id)
                    });
                    let _ = status_tracker.succeed_submission(&submission_id);
                }
                Ok(result) => {
                    let error_msg = result.error_message.unwrap_or_else(|| "Unknown error".to_string());
                    log_error!("Submission failed", {
                        "submission_id": &submission_id,
                        "provider_id": format!("{:?}", provider_id),
                        "error": &error_msg
                    });
                    let _ = status_tracker.fail_submission(
                        &submission_id,
                        SubmissionErrorType::ElementNotFound,
                        error_msg,
                    );
                }
                Err(e) => {
                    log_error!("Execution error", {
                        "submission_id": &submission_id,
                        "provider_id": format!("{:?}", provider_id),
                        "error": &e
                    });
                    let _ = status_tracker.fail_submission(
                        &submission_id,
                        SubmissionErrorType::NetworkError,
                        e,
                    );
                }
            }

            // T114: Event emission happens in status_tracker.update_status
            log_info!("Submission task completed", {
                "submission_id": &submission_id,
                "provider_id": format!("{:?}", provider_id)
            });
        });

        submissions.push(submission);
    }

    info!("Created and dispatched {} submissions", submissions.len());

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

