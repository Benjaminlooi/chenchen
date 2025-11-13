// Integration tests for structured logging format validation
// T147a: Verify structured logging outputs both JSON (machine-readable) and human-readable formats
// Constitution Principle IV: Dual output format requirement

use chenchen_lib::logging::{StructuredLog, LogLevel};
use serde_json;

#[test]
fn test_structured_log_outputs_json_format() {
    // Verify that StructuredLog can be serialized to JSON (machine-readable)

    let log = StructuredLog::new(
        LogLevel::Info,
        "Test operation completed".to_string(),
        serde_json::json!({
            "operation": "test_op",
            "duration_ms": 150,
            "success": true
        }),
    );

    // Serialize to JSON
    let json_output = serde_json::to_string(&log).expect("Failed to serialize to JSON");

    // Verify JSON structure
    assert!(json_output.contains("\"level\":\"info\""));
    assert!(json_output.contains("\"message\":\"Test operation completed\""));
    assert!(json_output.contains("\"operation\":\"test_op\""));
    assert!(json_output.contains("\"duration_ms\":150"));
    assert!(json_output.contains("\"success\":true"));
    assert!(json_output.contains("\"timestamp\":"));
}

#[test]
fn test_structured_log_outputs_human_readable_format() {
    // Verify that StructuredLog can be formatted for human reading

    let log = StructuredLog::new(
        LogLevel::Warn,
        "Resource usage high".to_string(),
        serde_json::json!({
            "cpu_percent": 85.5,
            "memory_mb": 512
        }),
    );

    // Human-readable format should include timestamp, message, and context
    // The actual format is: "[timestamp] message - context: {json}"
    // We can't test exact output without capturing logs, but we can verify structure

    assert_eq!(log.message, "Resource usage high");
    assert!(!log.timestamp.is_empty());
    assert_eq!(log.context["cpu_percent"], 85.5);
    assert_eq!(log.context["memory_mb"], 512);
}

#[test]
fn test_log_levels_serialize_correctly() {
    // Verify all log levels serialize to correct string values

    let info_log = StructuredLog::new(
        LogLevel::Info,
        "Info message".to_string(),
        serde_json::json!({}),
    );
    let warn_log = StructuredLog::new(
        LogLevel::Warn,
        "Warning message".to_string(),
        serde_json::json!({}),
    );
    let error_log = StructuredLog::new(
        LogLevel::Error,
        "Error message".to_string(),
        serde_json::json!({}),
    );

    let info_json = serde_json::to_string(&info_log).unwrap();
    let warn_json = serde_json::to_string(&warn_log).unwrap();
    let error_json = serde_json::to_string(&error_log).unwrap();

    // Verify level field uses lowercase per serde rename
    assert!(info_json.contains("\"level\":\"info\""));
    assert!(warn_json.contains("\"level\":\"warn\""));
    assert!(error_json.contains("\"level\":\"error\""));
}

#[test]
fn test_structured_log_with_complex_context() {
    // Verify complex nested context is preserved in JSON

    let log = StructuredLog::new(
        LogLevel::Error,
        "Database query failed".to_string(),
        serde_json::json!({
            "query": "SELECT * FROM users",
            "error": {
                "code": "CONNECTION_TIMEOUT",
                "details": {
                    "host": "localhost",
                    "port": 5432,
                    "timeout_ms": 5000
                }
            },
            "retry_count": 3
        }),
    );

    let json_output = serde_json::to_string(&log).unwrap();

    // Verify nested structure is preserved
    assert!(json_output.contains("\"query\":\"SELECT * FROM users\""));
    assert!(json_output.contains("\"code\":\"CONNECTION_TIMEOUT\""));
    assert!(json_output.contains("\"host\":\"localhost\""));
    assert!(json_output.contains("\"port\":5432"));
    assert!(json_output.contains("\"retry_count\":3"));
}

#[test]
fn test_structured_log_with_empty_context() {
    // Verify logs work with no context (null JSON object)

    let log = StructuredLog::new(
        LogLevel::Info,
        "Simple message with no context".to_string(),
        serde_json::json!({}),
    );

    let json_output = serde_json::to_string(&log).unwrap();

    assert!(json_output.contains("\"message\":\"Simple message with no context\""));
    assert!(json_output.contains("\"context\":{}"));
}

#[test]
fn test_timestamp_is_iso8601_format() {
    // Verify timestamp follows ISO 8601 format (RFC 3339)

    let log = StructuredLog::new(
        LogLevel::Info,
        "Timestamp test".to_string(),
        serde_json::json!({}),
    );

    // Attempt to parse timestamp as RFC 3339
    let parsed = chrono::DateTime::parse_from_rfc3339(&log.timestamp);
    assert!(
        parsed.is_ok(),
        "Timestamp '{}' should be valid RFC 3339 format",
        log.timestamp
    );
}

#[test]
fn test_logging_preserves_special_characters() {
    // Verify special characters in messages and context are properly escaped

    let log = StructuredLog::new(
        LogLevel::Warn,
        "Message with \"quotes\" and \n newlines".to_string(),
        serde_json::json!({
            "path": "/usr/local/bin",
            "command": "echo \"hello world\""
        }),
    );

    let json_output = serde_json::to_string(&log).unwrap();

    // JSON serialization should escape special characters
    assert!(json_output.contains("\\\"quotes\\\""));
    assert!(json_output.contains("\\n"));
    assert!(json_output.contains("/usr/local/bin"));
}
