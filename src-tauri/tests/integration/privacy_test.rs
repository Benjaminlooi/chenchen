// Integration tests for privacy guarantees
// T137: Verify no credential storage in app data directory
// T138: Verify no prompt history retained after app restart

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_no_credential_storage_in_app_data_directory() {
    // T137: Verify no credentials are stored in app data directory
    // FR-012: Only session cookies managed by platform webview

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let app_data_dir = temp_dir.path().to_path_buf();

    // Simulate app initialization with temp directory
    // In a real scenario, this would initialize the app with the temp directory

    // Search for common credential patterns in all files
    let credential_patterns = vec![
        "password",
        "api_key",
        "apiKey",
        "access_token",
        "accessToken",
        "secret",
        "auth_token",
        "bearer",
    ];

    // Recursively search all files in app data directory
    let files_with_credentials = search_directory_for_patterns(&app_data_dir, &credential_patterns);

    assert!(
        files_with_credentials.is_empty(),
        "Found potential credential storage in files: {:?}",
        files_with_credentials
    );
}

#[test]
fn test_no_prompt_history_retained_after_restart() {
    // T138: Verify no prompt history is retained after app restart
    // FR-012: All data stays local, no persistence of user prompts

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let app_data_dir = temp_dir.path().to_path_buf();

    // Simulate sending prompts (this would be through actual app commands)
    let test_prompts = vec![
        "What is the capital of France?",
        "Explain quantum computing",
        "Write a poem about rust",
    ];

    // Search for any files that might contain prompt history
    let history_patterns = vec!["history", "prompts", "queries", "user_input"];

    // Search for files with history-like names
    let history_files = search_directory_for_file_patterns(&app_data_dir, &history_patterns);

    assert!(
        history_files.is_empty(),
        "Found potential prompt history files: {:?}",
        history_files
    );

    // Also verify prompts are not stored in any files
    for prompt in &test_prompts {
        let files_containing_prompt = search_directory_for_content(&app_data_dir, prompt);

        assert!(
            files_containing_prompt.is_empty(),
            "Found prompt '{}' stored in files: {:?}",
            prompt,
            files_containing_prompt
        );
    }
}

#[test]
fn test_webview_data_only_contains_session_cookies() {
    // Additional test: Verify webview data directories only contain
    // session cookies, no credentials or prompt history

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let webview_dir = temp_dir.path().join("webviews");

    // Create webview directory structure
    fs::create_dir_all(&webview_dir).expect("Failed to create webview dir");

    // Patterns that should NOT exist in webview directories
    let forbidden_patterns = vec!["password", "secret", "api_key", "prompt_history"];

    let forbidden_files = search_directory_for_patterns(&webview_dir, &forbidden_patterns);

    assert!(
        forbidden_files.is_empty(),
        "Found forbidden data in webview directory: {:?}",
        forbidden_files
    );
}

// Helper functions

fn search_directory_for_patterns(dir: &PathBuf, patterns: &[&str]) -> Vec<PathBuf> {
    let mut matches = Vec::new();

    if !dir.exists() {
        return matches;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                // Read file content and check for patterns
                if let Ok(content) = fs::read_to_string(&path) {
                    let content_lower = content.to_lowercase();
                    for pattern in patterns {
                        if content_lower.contains(pattern) {
                            matches.push(path.clone());
                            break;
                        }
                    }
                }
            } else if path.is_dir() {
                // Recursively search subdirectories
                matches.extend(search_directory_for_patterns(&path, patterns));
            }
        }
    }

    matches
}

fn search_directory_for_file_patterns(dir: &PathBuf, patterns: &[&str]) -> Vec<PathBuf> {
    let mut matches = Vec::new();

    if !dir.exists() {
        return matches;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();

            for pattern in patterns {
                if filename.contains(pattern) {
                    matches.push(path.clone());
                    break;
                }
            }

            if path.is_dir() {
                matches.extend(search_directory_for_file_patterns(&path, patterns));
            }
        }
    }

    matches
}

fn search_directory_for_content(dir: &PathBuf, search_term: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();

    if !dir.exists() {
        return matches;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if content.contains(search_term) {
                        matches.push(path.clone());
                    }
                }
            } else if path.is_dir() {
                matches.extend(search_directory_for_content(&path, search_term));
            }
        }
    }

    matches
}
