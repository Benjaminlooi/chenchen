// Contract tests for authentication detection
// Tests WebviewManager's authentication check script generation

use chenchen_lib::types::ProviderId;
use chenchen_lib::webview::manager::WebviewManager;
use std::path::PathBuf;

#[test]
fn test_auth_check_script_generation() {
    let manager = WebviewManager::new(PathBuf::from("/tmp/test")).unwrap();

    let auth_selectors = vec![
        "button[data-testid='login-button']".to_string(),
        ".auth-required".to_string(),
    ];

    let script = manager.generate_auth_check_script(&auth_selectors);

    // Verify script contains the selectors
    assert!(script.contains("login-button"));
    assert!(script.contains("auth-required"));

    // Verify script structure
    assert!(script.contains("querySelector"));
    assert!(script.contains("is_authenticated"));
    assert!(script.contains("requires_login"));
}

#[test]
fn test_auth_check_script_empty_selectors() {
    let manager = WebviewManager::new(PathBuf::from("/tmp/test")).unwrap();

    let empty_selectors: Vec<String> = vec![];
    let script = manager.generate_auth_check_script(&empty_selectors);

    // With no selectors, script should still be valid JavaScript
    assert!(script.contains("function"));
    assert!(script.contains("is_authenticated"));
}

#[test]
fn test_create_auth_status_authenticated() {
    let manager = WebviewManager::new(PathBuf::from("/tmp/test")).unwrap();

    let status = manager.create_auth_status_mock(ProviderId::ChatGPT, true);

    assert_eq!(status.provider_id, ProviderId::ChatGPT);
    assert!(status.is_authenticated);
    assert!(!status.requires_login);
    assert!(!status.last_checked.is_empty());
}

#[test]
fn test_create_auth_status_unauthenticated() {
    let manager = WebviewManager::new(PathBuf::from("/tmp/test")).unwrap();

    let status = manager.create_auth_status_mock(ProviderId::Gemini, false);

    assert_eq!(status.provider_id, ProviderId::Gemini);
    assert!(!status.is_authenticated);
    assert!(status.requires_login);
    assert!(!status.last_checked.is_empty());
}

#[test]
fn test_auth_status_timestamp_format() {
    let manager = WebviewManager::new(PathBuf::from("/tmp/test")).unwrap();

    let status = manager.create_auth_status_mock(ProviderId::Claude, true);

    // Verify timestamp is ISO 8601 format (should parse as datetime)
    assert!(chrono::DateTime::parse_from_rfc3339(&status.last_checked).is_ok());
}
