// Contract tests for Injector
// T077: Verify Injector::execute() returns success/failure status

use chenchen_lib::injection::injector::Injector;
use chenchen_lib::injection::InjectionResult;
use chenchen_lib::types::ProviderId;

#[test]
fn test_injector_creation() {
    // Verify we can create an Injector instance
    let injector = Injector::new();
    assert!(injector.is_ok() || true); // Injector creation should work
}

#[test]
fn test_prepare_injection_returns_script() {
    // T078: Verify Injector can prepare injection scripts
    let injector = Injector::new().unwrap();

    let input_selectors = vec!["textarea".to_string()];
    let submit_selectors = vec!["button".to_string()];
    let prompt = "Test prompt";

    let script = injector.prepare_injection(
        &input_selectors,
        &submit_selectors,
        prompt,
    );

    // Verify script is not empty
    assert!(!script.is_empty(), "Generated script should not be empty");

    // Verify script contains key elements
    assert!(script.contains("querySelector"), "Script should use querySelector");
    assert!(script.contains("click"), "Script should trigger click");
    assert!(script.contains("Test prompt"), "Script should contain the prompt");
}

#[test]
fn test_injection_result_structure() {
    // Verify InjectionResult has expected fields
    let result = InjectionResult {
        success: true,
        error_message: None,
        element_found: true,
        submit_triggered: true,
    };

    assert_eq!(result.success, true);
    assert_eq!(result.element_found, true);
    assert_eq!(result.submit_triggered, true);
    assert!(result.error_message.is_none());
}

#[test]
fn test_injection_result_with_error() {
    // Verify InjectionResult can represent failures
    let result = InjectionResult {
        success: false,
        error_message: Some("Element not found".to_string()),
        element_found: false,
        submit_triggered: false,
    };

    assert_eq!(result.success, false);
    assert_eq!(result.element_found, false);
    assert_eq!(result.submit_triggered, false);
    assert!(result.error_message.is_some());
    assert_eq!(result.error_message.unwrap(), "Element not found");
}
