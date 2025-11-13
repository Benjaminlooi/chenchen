use crate::types::ProviderId;
use crate::webview::{WebviewInfo, AuthenticationStatus};
use std::path::PathBuf;

#[cfg(target_os = "macos")]
use std::collections::HashMap;
#[cfg(target_os = "macos")]
use uuid::Uuid;

/// Manages webview sessions for LLM providers
/// Handles platform-specific session persistence (data_directory for Windows/Linux, data_store_identifier for macOS)
pub struct WebviewManager {
    app_data_dir: PathBuf,
    #[cfg(target_os = "macos")]
    provider_identifiers: HashMap<ProviderId, String>,
}

impl WebviewManager {
    /// Creates a new WebviewManager with the given app data directory
    pub fn new(app_data_dir: PathBuf) -> Result<Self, String> {
        #[cfg(target_os = "macos")]
        {
            // On macOS, initialize persistent UUIDs for each provider
            let mut provider_identifiers = HashMap::new();

            // Generate or load persistent UUIDs
            // For now, we'll generate new ones each time (will be enhanced later with persistence)
            provider_identifiers.insert(ProviderId::ChatGPT, Uuid::new_v4().to_string());
            provider_identifiers.insert(ProviderId::Gemini, Uuid::new_v4().to_string());
            provider_identifiers.insert(ProviderId::Claude, Uuid::new_v4().to_string());

            Ok(Self {
                app_data_dir,
                provider_identifiers,
            })
        }

        #[cfg(not(target_os = "macos"))]
        {
            Ok(Self { app_data_dir })
        }
    }

    /// Gets the data directory path for a provider (Windows/Linux)
    #[cfg(not(target_os = "macos"))]
    pub fn get_data_directory(&self, provider_id: ProviderId) -> PathBuf {
        self.app_data_dir
            .join("webviews")
            .join(provider_id.as_str())
    }

    /// Gets the data store identifier for a provider (macOS)
    #[cfg(target_os = "macos")]
    pub fn get_data_store_identifier(&self, provider_id: ProviderId) -> String {
        self.provider_identifiers
            .get(&provider_id)
            .expect("Provider should have identifier")
            .clone()
    }

    /// Creates WebviewInfo for a provider with platform-specific configuration
    pub fn create_webview_info(&self, provider_id: ProviderId) -> WebviewInfo {
        let label = format!("{}-webview", provider_id.as_str().to_lowercase());
        let url = provider_id.url().to_string();

        #[cfg(not(target_os = "macos"))]
        {
            let data_path = self.get_data_directory(provider_id);
            WebviewInfo {
                provider_id,
                label,
                url,
                is_persistent: true,
                data_path: data_path.to_string_lossy().to_string(),
            }
        }

        #[cfg(target_os = "macos")]
        {
            let data_store_id = self.get_data_store_identifier(provider_id);
            WebviewInfo {
                provider_id,
                label,
                url,
                is_persistent: true,
                data_store_id,
            }
        }
    }

    /// Ensures data directory exists for a provider (Windows/Linux only)
    #[cfg(not(target_os = "macos"))]
    pub fn ensure_data_directory(&self, provider_id: ProviderId) -> Result<PathBuf, std::io::Error> {
        let data_dir = self.get_data_directory(provider_id);
        std::fs::create_dir_all(&data_dir)?;
        Ok(data_dir)
    }

    /// Generates JavaScript to check authentication status
    /// Returns script that checks for presence of auth_check_selectors
    ///
    /// # Arguments
    /// * `auth_check_selectors` - CSS selectors that indicate unauthenticated state
    ///
    /// # Returns
    /// JavaScript code that returns true if authenticated (selectors NOT found)
    pub fn generate_auth_check_script(&self, auth_check_selectors: &[String]) -> String {
        let selectors_json = serde_json::to_string(auth_check_selectors).unwrap_or_else(|_| "[]".to_string());

        format!(
            r#"
(function() {{
    try {{
        const authCheckSelectors = {selectors};

        // Check if any auth-required selector is present
        for (let i = 0; i < authCheckSelectors.length; i++) {{
            const selector = authCheckSelectors[i];
            const element = document.querySelector(selector);
            if (element) {{
                console.log('Auth check: Found auth-required element with selector:', selector);
                return {{ is_authenticated: false, requires_login: true }};
            }}
        }}

        console.log('Auth check: No auth-required elements found');
        return {{ is_authenticated: true, requires_login: false }};
    }} catch (error) {{
        console.error('Auth check error:', error);
        return {{ is_authenticated: false, requires_login: true, error: error.message }};
    }}
}})();
"#,
            selectors = selectors_json
        )
    }

    /// Creates a mock authentication status for testing
    /// In production, this would execute the auth check script in a webview
    pub fn create_auth_status_mock(&self, provider_id: ProviderId, is_authenticated: bool) -> AuthenticationStatus {
        AuthenticationStatus {
            provider_id,
            is_authenticated,
            last_checked: chrono::Utc::now().to_rfc3339(),
            requires_login: !is_authenticated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webview_manager_creation() {
        let manager = WebviewManager::new(PathBuf::from("/tmp/test"));
        assert!(manager.is_ok());
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn test_data_directory_path_format() {
        let manager = WebviewManager::new(PathBuf::from("/tmp/test")).unwrap();
        let path = manager.get_data_directory(ProviderId::ChatGPT);

        assert!(path.to_string_lossy().contains("webviews"));
        assert!(path.to_string_lossy().contains("ChatGPT"));
    }

    #[test]
    fn test_create_webview_info_labels() {
        let manager = WebviewManager::new(PathBuf::from("/tmp/test")).unwrap();

        let chatgpt_info = manager.create_webview_info(ProviderId::ChatGPT);
        assert_eq!(chatgpt_info.label, "chatgpt-webview");

        let gemini_info = manager.create_webview_info(ProviderId::Gemini);
        assert_eq!(gemini_info.label, "gemini-webview");

        let claude_info = manager.create_webview_info(ProviderId::Claude);
        assert_eq!(claude_info.label, "claude-webview");
    }
}
