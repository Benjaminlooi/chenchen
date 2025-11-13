use std::sync::Mutex;

/// Application state shared across Tauri commands
/// This state is managed by Tauri and accessible to all commands
#[derive(Default)]
pub struct AppState {
    // Placeholder for future state management
    // Will hold:
    // - Provider configurations
    // - Active submissions
    // - Webview sessions
    // - Layout configuration
    _placeholder: Mutex<()>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            _placeholder: Mutex::new(()),
        }
    }
}
