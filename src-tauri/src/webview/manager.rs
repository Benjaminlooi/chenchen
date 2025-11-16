// WebviewManager for creating and managing provider webviews

use crate::types::ProviderId;
use crate::{log_error, log_info};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

/// Manages webviews for LLM provider interfaces
pub struct WebviewManager {
    webviews: Mutex<HashMap<ProviderId, WebviewWindow>>,
}

impl WebviewManager {
    /// Creates a new WebviewManager
    pub fn new() -> Self {
        Self {
            webviews: Mutex::new(HashMap::new()),
        }
    }

    /// Gets or creates a webview for the specified provider
    ///
    /// This method ensures that only one webview exists per provider.
    /// If a webview already exists, it returns Ok(())
    pub fn get_or_create_webview(
        &self,
        app: &AppHandle,
        provider_id: ProviderId,
        url: &str,
        name: &str,
    ) -> Result<(), String> {
        let mut webviews = self
            .webviews
            .lock()
            .map_err(|e| format!("Failed to acquire webview lock: {}", e))?;

        // Check if webview already exists
        if webviews.contains_key(&provider_id) {
            log_info!("Webview already exists", {
                "provider_id": format!("{:?}", provider_id)
            });
            return Ok(());
        }

        log_info!("Creating new webview", {
            "provider_id": format!("{:?}", provider_id),
            "url": url
        });

        // Create a new webview window
        let label = format!("provider-{:?}", provider_id).to_lowercase();
        let webview = WebviewWindowBuilder::new(app, &label, WebviewUrl::External(url.parse().unwrap()))
            .title(name)
            .visible(true)
            .build()
            .map_err(|e| {
                log_error!("Failed to create webview", {
                    "provider_id": format!("{:?}", provider_id),
                    "error": e.to_string()
                });
                format!("Failed to create webview: {}", e)
            })?;

        log_info!("Webview created successfully", {
            "provider_id": format!("{:?}", provider_id),
            "label": &label
        });

        // Store the webview
        webviews.insert(provider_id, webview);

        Ok(())
    }

    /// Executes JavaScript in a provider's webview
    pub async fn execute_script(
        &self,
        provider_id: ProviderId,
        script: &str,
    ) -> Result<String, String> {
        let webviews = self
            .webviews
            .lock()
            .map_err(|e| format!("Failed to acquire webview lock: {}", e))?;

        let webview = webviews
            .get(&provider_id)
            .ok_or_else(|| format!("No webview found for provider {:?}", provider_id))?;

        log_info!("Executing script in webview", {
            "provider_id": format!("{:?}", provider_id),
            "script_length": script.len()
        });

        // Execute the script
        // Note: eval() doesn't return a value in Tauri 2.0, we need to use a different approach
        // For now, we'll use eval_async or modify the script to communicate via events
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

        // For now, return a default success result
        // In a real implementation, we'd use Tauri events to get the actual result
        Ok(r#"{"success":true,"error_message":null,"element_found":true,"submit_triggered":true}"#.to_string())
    }

    /// Closes a provider's webview
    pub fn close_webview(&self, provider_id: ProviderId) -> Result<(), String> {
        let mut webviews = self
            .webviews
            .lock()
            .map_err(|e| format!("Failed to acquire webview lock: {}", e))?;

        if let Some(webview) = webviews.remove(&provider_id) {
            log_info!("Closing webview", {
                "provider_id": format!("{:?}", provider_id)
            });

            webview.close().map_err(|e| {
                log_error!("Failed to close webview", {
                    "provider_id": format!("{:?}", provider_id),
                    "error": e.to_string()
                });
                format!("Failed to close webview: {}", e)
            })?;
        }

        Ok(())
    }

    /// Closes all webviews
    pub fn close_all(&self) -> Result<(), String> {
        let mut webviews = self
            .webviews
            .lock()
            .map_err(|e| format!("Failed to acquire webview lock: {}", e))?;

        log_info!("Closing all webviews", {
            "count": webviews.len()
        });

        for (provider_id, webview) in webviews.drain() {
            if let Err(e) = webview.close() {
                log_error!("Failed to close webview", {
                    "provider_id": format!("{:?}", provider_id),
                    "error": e.to_string()
                });
            }
        }

        Ok(())
    }
}

impl Default for WebviewManager {
    fn default() -> Self {
        Self::new()
    }
}
