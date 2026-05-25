use log::{error, info, warn};
use tauri::Manager;

// Module declarations
pub mod commands;
pub mod injection;
pub mod layout;
pub mod logging;
pub mod providers;
pub mod state;
pub mod status;
pub mod types;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting ChenChen application");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let provider_preferences_path = match app.path().app_config_dir() {
                Ok(config_dir) => Some(config_dir.join("provider-preferences.json")),
                Err(e) => {
                    warn!("Failed to resolve app config directory: {}", e);
                    None
                }
            };

            app.manage(AppState::new(provider_preferences_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_providers,
            commands::update_provider_selection,
            commands::get_layout_configuration,
            commands::submit_prompt,
            commands::get_submission_status,
            commands::report_execution_result,
            commands::sync_provider_webview,
            commands::dispose_provider_webview,
            commands::refresh_provider_webview,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|err| {
            error!("Error running Tauri application: {}", err);
        });
}
