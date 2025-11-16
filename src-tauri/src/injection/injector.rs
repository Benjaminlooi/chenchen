// Injector for executing JavaScript in webview contexts
// Handles script execution, timeout management, and result parsing

use super::script_builder;
use super::InjectionResult;
use crate::log_error;
use crate::log_info;
use crate::types::ProviderId;
use crate::webview::WebviewManager;
use tauri::AppHandle;

/// Manages JavaScript injection into provider webviews
pub struct Injector {}

impl Injector {
    /// Creates a new Injector instance
    pub fn new() -> Result<Self, String> {
        Ok(Self {})
    }

    /// Prepares an injection script for execution
    ///
    /// This generates the JavaScript code but does not execute it.
    /// Use this to prepare scripts that will be executed later via webview.eval()
    ///
    /// # Arguments
    /// * `input_selectors` - CSS selectors for finding the prompt input element
    /// * `submit_selectors` - CSS selectors for finding the submit button
    /// * `prompt` - The prompt text to inject
    ///
    /// # Returns
    /// A JavaScript code string ready for execution
    pub fn prepare_injection(
        &self,
        input_selectors: &[String],
        submit_selectors: &[String],
        prompt: &str,
    ) -> String {
        log_info!("Preparing injection script", {
            "input_selectors_count": input_selectors.len(),
            "submit_selectors_count": submit_selectors.len(),
            "prompt_length": prompt.len()
        });

        let script =
            script_builder::generate_injection_script(input_selectors, submit_selectors, prompt);

        log_info!("Injection script generated", {
            "script_length": script.len()
        });

        script
    }

    /// Executes an injection script in an existing webview
    ///
    /// This executes the JavaScript code in the provider's webview and parses the result.
    /// The script should return a JSON object with the InjectionResult structure.
    ///
    /// # Arguments
    /// * `app` - The Tauri app handle
    /// * `webview_manager` - The webview manager
    /// * `provider_id` - The provider to execute the script for
    /// * `script` - The JavaScript code to execute
    ///
    /// # Returns
    /// Result containing the parsed InjectionResult or an error message
    pub async fn execute(
        &self,
        app: &AppHandle,
        webview_manager: &WebviewManager,
        provider_id: ProviderId,
        script: &str,
    ) -> Result<InjectionResult, String> {
        log_info!("Executing injection script", {
            "provider_id": format!("{:?}", provider_id),
            "script_length": script.len()
        });

        // Execute the script in the existing webview
        let result_str = webview_manager
            .execute_script(app, provider_id, script)
            .await?;

        log_info!("Raw script result", {
            "provider_id": format!("{:?}", provider_id),
            "result": &result_str
        });

        // Parse the JSON result
        let result: InjectionResult = serde_json::from_str(&result_str).map_err(|e| {
            log_error!("Failed to parse injection result", {
                "provider_id": format!("{:?}", provider_id),
                "error": e.to_string(),
                "result": &result_str
            });
            format!("Failed to parse injection result: {}", e)
        })?;

        log_info!("Injection execution completed", {
            "provider_id": format!("{:?}", provider_id),
            "success": result.success,
            "element_found": result.element_found,
            "submit_triggered": result.submit_triggered
        });

        Ok(result)
    }

    /// Executes an injection script in a webview (mock version for testing)
    ///
    /// NOTE: This is kept for backward compatibility with tests.
    /// Use execute() for real execution.
    #[cfg(test)]
    pub fn execute_mock(&self, _script: &str) -> Result<InjectionResult, String> {
        log_info!("Executing injection script (mock)", {
            "script_length": _script.len()
        });

        let result = InjectionResult {
            success: true,
            error_message: None,
            element_found: true,
            submit_triggered: true,
        };

        log_info!("Injection execution completed", {
            "success": result.success,
            "element_found": result.element_found,
            "submit_triggered": result.submit_triggered
        });

        Ok(result)
    }
}

impl Default for Injector {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injector_creation() {
        let injector = Injector::new();
        assert!(injector.is_ok());
    }

    #[test]
    fn test_prepare_injection_generates_script() {
        let injector = Injector::new().unwrap();
        let script = injector.prepare_injection(
            &vec!["input".to_string()],
            &vec!["button".to_string()],
            "Test",
        );

        assert!(!script.is_empty());
        assert!(script.contains("querySelector"));
    }

    #[test]
    fn test_execute_mock_returns_success() {
        let injector = Injector::new().unwrap();
        let result = injector.execute_mock("mock script");

        assert!(result.is_ok());
        let injection_result = result.unwrap();
        assert!(injection_result.success);
    }
}
