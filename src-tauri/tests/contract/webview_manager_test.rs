// Contract tests for WebviewManager
// These tests verify the public API interface for webview management

use chenchen_lib::types::ProviderId;
use chenchen_lib::webview::manager::WebviewManager;
use std::path::PathBuf;

#[test]
fn test_new_creates_manager() {
    let app_data_dir = PathBuf::from("/tmp/test_chenchen");
    let manager = WebviewManager::new(app_data_dir);

    assert!(manager.is_ok());
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_get_data_directory_for_provider_windows_linux() {
    let app_data_dir = PathBuf::from("/tmp/test_chenchen");
    let manager = WebviewManager::new(app_data_dir.clone()).unwrap();

    let data_dir = manager.get_data_directory(ProviderId::ChatGPT);

    assert!(data_dir.to_string_lossy().contains("test_chenchen"));
    assert!(data_dir.to_string_lossy().contains("webviews"));
    assert!(data_dir.to_string_lossy().contains("ChatGPT"));
}

#[test]
#[cfg(target_os = "macos")]
fn test_get_data_store_identifier_for_provider_macos() {
    let app_data_dir = PathBuf::from("/tmp/test_chenchen");
    let manager = WebviewManager::new(app_data_dir).unwrap();

    let identifier1 = manager.get_data_store_identifier(ProviderId::ChatGPT);
    let identifier2 = manager.get_data_store_identifier(ProviderId::ChatGPT);

    // Same provider should return same identifier (persistence)
    assert_eq!(identifier1, identifier2);

    // Different providers should have different identifiers
    let identifier_gemini = manager.get_data_store_identifier(ProviderId::Gemini);
    assert_ne!(identifier1, identifier_gemini);
}

#[test]
fn test_create_webview_info() {
    let app_data_dir = PathBuf::from("/tmp/test_chenchen");
    let manager = WebviewManager::new(app_data_dir).unwrap();

    let webview_info = manager.create_webview_info(ProviderId::Claude);

    assert_eq!(webview_info.provider_id, ProviderId::Claude);
    assert_eq!(webview_info.label, "webviews/Claude");
    assert_eq!(webview_info.url, "https://claude.ai/");
    assert_eq!(webview_info.is_persistent, true);

    #[cfg(not(target_os = "macos"))]
    {
        assert!(webview_info.data_path.contains("Claude"));
    }

    #[cfg(target_os = "macos")]
    {
        assert!(!webview_info.data_store_id.is_empty());
    }
}

#[test]
fn test_all_providers_get_unique_sessions() {
    let app_data_dir = PathBuf::from("/tmp/test_chenchen");
    let manager = WebviewManager::new(app_data_dir).unwrap();

    let chatgpt_info = manager.create_webview_info(ProviderId::ChatGPT);
    let gemini_info = manager.create_webview_info(ProviderId::Gemini);
    let claude_info = manager.create_webview_info(ProviderId::Claude);

    // All providers should have unique labels
    assert_ne!(chatgpt_info.label, gemini_info.label);
    assert_ne!(gemini_info.label, claude_info.label);
    assert_ne!(chatgpt_info.label, claude_info.label);

    #[cfg(not(target_os = "macos"))]
    {
        // All providers should have unique data directories
        assert_ne!(chatgpt_info.data_path, gemini_info.data_path);
        assert_ne!(gemini_info.data_path, claude_info.data_path);
    }

    #[cfg(target_os = "macos")]
    {
        // All providers should have unique data store identifiers
        assert_ne!(chatgpt_info.data_store_id, gemini_info.data_store_id);
        assert_ne!(gemini_info.data_store_id, claude_info.data_store_id);
    }
}
