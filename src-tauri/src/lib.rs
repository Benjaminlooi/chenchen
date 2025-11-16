use log::{error, info};

// Module declarations
pub mod commands;
pub mod injection;
pub mod layout;
pub mod logging;
pub mod providers;
pub mod state;
pub mod status;
pub mod types;
pub mod webview;

use state::AppState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    info!("Greet command called with name: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting ChenChen application");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new()) // Register shared application state
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::get_providers,
            commands::update_provider_selection,
            commands::get_layout_configuration,
            commands::submit_prompt,
            commands::get_submission_status,
            commands::report_execution_result,
            commands::sync_provider_webview,
            commands::dispose_provider_webview,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|err| {
            error!("Error running Tauri application: {}", err);
        });
}
