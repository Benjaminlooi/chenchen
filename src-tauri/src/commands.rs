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

fn adjust_child_webview_bounds_for_window_chrome(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    outer_height: u32,
    inner_height: u32,
    scale_factor: f64,
    apply_native_chrome_offset: bool,
) -> (f64, f64, f64, f64) {
    #[allow(unused_mut)]
    let mut chrome_top_offset = if apply_native_chrome_offset && scale_factor > 0.0 {
        outer_height.saturating_sub(inner_height) as f64 / scale_factor
    } else {
        0.0
    };

    #[cfg(target_os = "macos")]
    if apply_native_chrome_offset && chrome_top_offset == 0.0 && outer_height >= inner_height {
        chrome_top_offset = 53.0;
    }

    let adjusted_height = (height - chrome_top_offset).max(1.0);

    (x, y + chrome_top_offset, width, adjusted_height)
}

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
/// Creates submissions, generates injection scripts, and emits events for frontend execution
///
/// Returns the created submissions with their IDs for status tracking
#[tauri::command]
pub async fn submit_prompt(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    prompt: String,
) -> Result<Vec<crate::status::Submission>, CommandError> {
    use crate::injection::injector::Injector;
    use std::sync::Arc;

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

    // Create submissions for each selected provider
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

        // Start the submission
        state.status_tracker.start_submission(&submission.id)?;

        // Get provider config for selectors
        let config = provider_configs.get_config(provider.id)?;

        // Generate injection script
        let script =
            injector.prepare_injection(&config.input_selectors, &config.submit_selectors, &prompt);

        info!(
            "Generated injection script for provider {:?} ({} chars)",
            provider.id,
            script.len()
        );

        // Execute the script in the webview
        let label = format!("{}-webview", provider.id.as_str().to_lowercase());
        let submission_id = submission.id.clone();
        let provider_id = provider.id;

        // Clone for async execution
        let app_clone = app.clone();
        let script_clone = script.clone();
        let status_tracker = Arc::clone(&state.status_tracker);

        // Spawn async task to execute
        tauri::async_runtime::spawn(async move {
            use tauri::Manager;

            // Get the webview (child webview, not window)
            let webview = match app_clone.get_webview(&label) {
                Some(wv) => wv,
                None => {
                    log_error!("Webview not found for execution", {
                        "submission_id": &submission_id,
                        "provider_id": format!("{:?}", provider_id),
                        "label": &label
                    });
                    let _ = status_tracker.fail_submission(
                        &submission_id,
                        crate::types::SubmissionErrorType::InjectionFailed,
                        format!("Webview not found: {}", label),
                    );
                    return;
                }
            };

            // Execute the script
            match webview.eval(&script_clone) {
                Ok(_) => {
                    log_info!("Script executed successfully", {
                        "submission_id": &submission_id,
                        "provider_id": format!("{:?}", provider_id)
                    });
                    let _ = status_tracker.succeed_submission(&submission_id);
                }
                Err(e) => {
                    log_error!("Script execution failed", {
                        "submission_id": &submission_id,
                        "provider_id": format!("{:?}", provider_id),
                        "error": e.to_string()
                    });
                    let _ = status_tracker.fail_submission(
                        &submission_id,
                        crate::types::SubmissionErrorType::InjectionFailed,
                        e.to_string(),
                    );
                }
            }
        });

        submissions.push(submission);
    }

    info!("Created and dispatched {} submissions", submissions.len());

    Ok(submissions)
}

/// Creates or updates a provider webview
#[tauri::command]
pub async fn sync_provider_webview(
    app: tauri::AppHandle,
    provider_id: ProviderId,
    url: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), CommandError> {
    use tauri::{Manager, WebviewBuilder, WebviewUrl};
    use tauri::{Position, Rect, Size};

    let label = format!("{}-webview", provider_id.as_str().to_lowercase());

    log_info!("Syncing provider webview", {
        "provider_id": format!("{:?}", provider_id),
        "label": &label,
        "position": format!("({}, {})", x, y),
        "size": format!("{}x{}", width, height)
    });

    // Get the main window to use as parent (bare Window, not WebviewWindow)
    let main_window = app
        .get_window("main")
        .ok_or_else(|| CommandError::internal("Main window not found"))?;

    let outer_size = main_window
        .outer_size()
        .map_err(|e| CommandError::internal(format!("Failed to get outer window size: {}", e)))?;
    let inner_size = main_window
        .inner_size()
        .map_err(|e| CommandError::internal(format!("Failed to get inner window size: {}", e)))?;
    let scale_factor = main_window
        .scale_factor()
        .map_err(|e| CommandError::internal(format!("Failed to get window scale factor: {}", e)))?;

    log_info!("Window size details for webview positioning", {
        "outer_height": outer_size.height,
        "inner_height": inner_size.height,
        "scale_factor": scale_factor
    });
    let (x, y, width, height) = adjust_child_webview_bounds_for_window_chrome(
        x,
        y,
        width,
        height,
        outer_size.height,
        inner_size.height,
        scale_factor,
        cfg!(target_os = "macos"),
    );

    // Check if webview already exists
    if let Some(webview) = app.get_webview(&label) {
        // Update bounds using set_bounds for proper GTK Fixed container handling
        let bounds = Rect {
            position: Position::Logical(tauri::LogicalPosition { x, y }),
            size: Size::Logical(tauri::LogicalSize { width, height }),
        };

        webview
            .set_bounds(bounds)
            .map_err(|e| CommandError::internal(format!("Failed to set bounds: {}", e)))?;

        log_info!("Updated existing webview bounds", {
            "label": &label,
            "adjusted_position": format!("({}, {})", x, y),
            "adjusted_size": format!("{}x{}", width, height)
        });
    } else {
        // Create new child webview attached to main window
        // T155: Set User Agent to fix Gemini icons on Linux
        // T160: Inject stealth script to prevent bot detection
        let stealth_script = crate::injection::injector::Injector::get_stealth_script();

        // T161: Platform-specific User-Agent corresponding to the stealth script
        #[cfg(target_os = "macos")]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15")
            .initialization_script(stealth_script);

        #[cfg(not(target_os = "macos"))]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .initialization_script(stealth_script);

        let position = tauri::LogicalPosition { x, y };
        let size = tauri::LogicalSize { width, height };

        let _webview = main_window
            .add_child(
                webview_builder,
                Position::Logical(position),
                Size::Logical(size),
            )
            .map_err(|e| {
                log_error!("Failed to create child webview", {
                    "label": &label,
                    "error": e.to_string()
                });
                CommandError::internal(format!("Failed to create child webview: {}", e))
            })?;

        log_info!("Created new child webview", {
            "label": &label,
            "adjusted_position": format!("({}, {})", x, y),
            "adjusted_size": format!("{}x{}", width, height)
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::adjust_child_webview_bounds_for_window_chrome;

    #[test]
    fn leaves_bounds_unchanged_without_native_chrome() {
        let bounds = adjust_child_webview_bounds_for_window_chrome(
            12.0, 34.0, 500.0, 600.0, 900, 900, 1.0, false,
        );

        assert_eq!(bounds, (12.0, 34.0, 500.0, 600.0));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn applies_macos_titlebar_fallback_when_window_sizes_match() {
        let bounds = adjust_child_webview_bounds_for_window_chrome(
            12.0, 34.0, 500.0, 600.0, 900, 900, 1.0, true,
        );

        assert_eq!(bounds, (12.0, 87.0, 500.0, 547.0));
    }

    #[test]
    fn keeps_child_webview_y_aligned_to_content_rect() {
        let bounds = adjust_child_webview_bounds_for_window_chrome(
            12.0, 34.0, 500.0, 600.0, 1800, 1712, 2.0, true,
        );

        assert_eq!(bounds, (12.0, 78.0, 500.0, 556.0));
    }

    #[test]
    fn leaves_bounds_unchanged_when_native_chrome_offset_is_disabled() {
        let bounds = adjust_child_webview_bounds_for_window_chrome(
            12.0, 34.0, 500.0, 600.0, 1800, 1712, 2.0, false,
        );

        assert_eq!(bounds, (12.0, 34.0, 500.0, 600.0));
    }

    #[test]
    fn does_not_underflow_when_inner_size_exceeds_outer_size() {
        let bounds = adjust_child_webview_bounds_for_window_chrome(
            12.0, 34.0, 500.0, 600.0, 890, 900, 1.0, true,
        );

        assert_eq!(bounds, (12.0, 34.0, 500.0, 600.0));
    }

    #[test]
    fn clamps_height_when_native_chrome_offset_exceeds_target_height() {
        let bounds = adjust_child_webview_bounds_for_window_chrome(
            12.0, 34.0, 500.0, 20.0, 1800, 1712, 2.0, true,
        );

        assert_eq!(bounds, (12.0, 78.0, 500.0, 1.0));
    }
}

/// Disposes an existing provider webview when it is no longer needed
#[tauri::command]
pub async fn dispose_provider_webview(
    app: tauri::AppHandle,
    provider_id: ProviderId,
) -> Result<(), CommandError> {
    use tauri::Manager;

    let label = format!("{}-webview", provider_id.as_str().to_lowercase());

    log_info!("Disposing provider webview", {
        "provider_id": format!("{:?}", provider_id),
        "label": &label
    });

    if let Some(webview) = app.get_webview(&label) {
        webview
            .close()
            .map_err(|e| CommandError::internal(format!("Failed to close webview: {}", e)))?;

        log_info!("Closed provider webview", {
            "label": &label
        });
    } else {
        log_info!("No provider webview found to dispose", {
            "label": &label
        });
    }

    Ok(())
}

/// Refreshes an existing provider webview
#[tauri::command]
pub async fn refresh_provider_webview(
    app: tauri::AppHandle,
    provider_id: ProviderId,
) -> Result<(), CommandError> {
    use tauri::Manager;

    let label = format!("{}-webview", provider_id.as_str().to_lowercase());

    log_info!("Refreshing provider webview", {
        "provider_id": format!("{:?}", provider_id),
        "label": &label
    });

    if let Some(webview) = app.get_webview(&label) {
        webview
            .reload()
            .map_err(|e| CommandError::internal(format!("Failed to reload webview: {}", e)))?;

        log_info!("Reloaded provider webview", {
            "label": &label
        });
    } else {
        log_info!("No provider webview found to refresh", {
            "label": &label
        });
    }

    Ok(())
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

/// Handles execution results from the frontend
#[tauri::command]
pub fn report_execution_result(
    state: State<AppState>,
    payload: crate::types::ExecutionResultPayload,
) -> Result<(), CommandError> {
    use crate::types::SubmissionErrorType;

    log_info!("Command: report_execution_result called", {
        "submission_id": &payload.submission_id,
        "provider_id": format!("{:?}", payload.provider_id),
        "success": payload.success
    });

    // Update submission status based on the result
    if payload.success {
        state
            .status_tracker
            .succeed_submission(&payload.submission_id)?;

        log_info!("Marked submission as successful", {
            "submission_id": &payload.submission_id
        });
    } else {
        let error_type = if !payload.element_found {
            SubmissionErrorType::ElementNotFound
        } else {
            SubmissionErrorType::InjectionFailed
        };

        state.status_tracker.fail_submission(
            &payload.submission_id,
            error_type,
            payload
                .error_message
                .unwrap_or_else(|| "Execution failed".to_string()),
        )?;

        log_info!("Marked submission as failed", {
            "submission_id": &payload.submission_id,
            "error_type": format!("{:?}", error_type)
        });
    }

    Ok(())
}
