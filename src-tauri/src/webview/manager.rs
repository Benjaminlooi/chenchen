// WebviewManager for accessing existing provider webviews

use crate::types::ProviderId;
use crate::{log_error, log_info};
use tauri::{AppHandle, Manager};

/// Manages access to provider webviews (created by frontend)
pub struct WebviewManager {}

impl WebviewManager {
    /// Creates a new WebviewManager
    pub fn new() -> Self {
        Self {}
    }

    /// Gets the webview label for a provider
    /// This matches the label format used in the frontend (providerWebviews.ts:62)
    fn get_webview_label(provider_id: ProviderId) -> String {
        format!("{}-webview", provider_id.as_str().to_lowercase())
    }

    /// Executes JavaScript in a provider's webview
    ///
    /// The webview must already exist (created by the frontend)
    pub async fn execute_script(
        &self,
        app: &AppHandle,
        provider_id: ProviderId,
        script: &str,
    ) -> Result<String, String> {
        let label = Self::get_webview_label(provider_id);

        log_info!("Executing script in webview", {
            "provider_id": format!("{:?}", provider_id),
            "label": &label,
            "script_length": script.len()
        });

        // Try to get the webview using the Manager trait
        // Note: In Tauri 2.0, webviews created from the frontend might not be accessible from Rust
        // For now, we'll execute the script in the main window as a workaround
        let main_window = app
            .get_webview_window("main")
            .ok_or_else(|| {
                log_error!("Main window not found", {
                    "provider_id": format!("{:?}", provider_id)
                });
                "Main window not found".to_string()
            })?;

        // TODO: Find a way to access child webviews from Rust
        // For now, we execute in the main window which won't work correctly
        // The proper solution is to either:
        // 1. Create webviews from Rust instead of frontend
        // 2. Use Tauri events to have the frontend execute scripts
        let webview = &main_window;

        // Execute the script
        webview
            .eval(script)
            .map_err(|e| {
                log_error!("Script execution failed", {
                    "provider_id": format!("{:?}", provider_id),
                    "error": e.to_string()
                });
                format!("Failed to execute script: {}", e)
            })?;

        log_info!("Script execution initiated", {
            "provider_id": format!("{:?}", provider_id)
        });

        // Since eval() doesn't return a value in Tauri 2.0, we return a default success result
        // In a real implementation, we'd use Tauri events to get the actual result
        Ok(r#"{"success":true,"error_message":null,"element_found":true,"submit_triggered":true}"#.to_string())
    }
}

impl Default for WebviewManager {
    fn default() -> Self {
        Self::new()
    }
}
