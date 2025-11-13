// Structured logging utilities
// Constitution Principle IV: Dual output format (JSON + human-readable)

use serde::{Serialize, Deserialize};
use serde_json;
use log::{info, warn, error};

/// Log levels matching the standard log crate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

/// Structured log entry that can be serialized to JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLog {
    pub level: LogLevel,
    pub message: String,
    pub context: serde_json::Value,
    pub timestamp: String,
}

impl StructuredLog {
    pub fn new(level: LogLevel, message: String, context: serde_json::Value) -> Self {
        Self {
            level,
            message,
            context,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Outputs to logger in both JSON and human-readable formats
    pub fn emit(&self) {
        // JSON format (machine-readable)
        let json_output = serde_json::to_string(self)
            .unwrap_or_else(|_| format!("{{\"error\": \"Failed to serialize log\", \"message\": \"{}\"}}", self.message));

        // Human-readable format
        let human_output = format!(
            "[{}] {} - {}",
            self.timestamp,
            self.message,
            if self.context.is_null() {
                String::new()
            } else {
                format!("context: {}", self.context)
            }
        );

        // Emit both formats based on log level
        match self.level {
            LogLevel::Info => {
                info!("{}", human_output);
                info!(target: "structured", "{}", json_output);
            }
            LogLevel::Warn => {
                warn!("{}", human_output);
                warn!(target: "structured", "{}", json_output);
            }
            LogLevel::Error => {
                error!("{}", human_output);
                error!(target: "structured", "{}", json_output);
            }
        }
    }
}

/// Helper macro for structured info logging
#[macro_export]
macro_rules! log_info {
    ($message:expr, { $($key:tt : $value:expr),* $(,)? }) => {{
        let log_entry = $crate::logging::StructuredLog::new(
            $crate::logging::LogLevel::Info,
            $message.to_string(),
            serde_json::json!({ $($key: $value),* }),
        );
        log_entry.emit();
    }};
    ($message:expr) => {{
        let log_entry = $crate::logging::StructuredLog::new(
            $crate::logging::LogLevel::Info,
            $message.to_string(),
            serde_json::json!({}),
        );
        log_entry.emit();
    }};
}

/// Helper macro for structured warning logging
#[macro_export]
macro_rules! log_warn {
    ($message:expr, { $($key:tt : $value:expr),* $(,)? }) => {{
        let log_entry = $crate::logging::StructuredLog::new(
            $crate::logging::LogLevel::Warn,
            $message.to_string(),
            serde_json::json!({ $($key: $value),* }),
        );
        log_entry.emit();
    }};
    ($message:expr) => {{
        let log_entry = $crate::logging::StructuredLog::new(
            $crate::logging::LogLevel::Warn,
            $message.to_string(),
            serde_json::json!({}),
        );
        log_entry.emit();
    }};
}

/// Helper macro for structured error logging
#[macro_export]
macro_rules! log_error {
    ($message:expr, { $($key:tt : $value:expr),* $(,)? }) => {{
        let log_entry = $crate::logging::StructuredLog::new(
            $crate::logging::LogLevel::Error,
            $message.to_string(),
            serde_json::json!({ $($key: $value),* }),
        );
        log_entry.emit();
    }};
    ($message:expr) => {{
        let log_entry = $crate::logging::StructuredLog::new(
            $crate::logging::LogLevel::Error,
            $message.to_string(),
            serde_json::json!({}),
        );
        log_entry.emit();
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_log_creation() {
        let log = StructuredLog::new(
            LogLevel::Info,
            "Test message".to_string(),
            serde_json::json!({"key": "value"}),
        );

        assert_eq!(log.message, "Test message");
        assert!(!log.timestamp.is_empty());
    }

    #[test]
    fn test_structured_log_serialization() {
        let log = StructuredLog::new(
            LogLevel::Error,
            "Error occurred".to_string(),
            serde_json::json!({"error_code": 500}),
        );

        let json = serde_json::to_string(&log).expect("Failed to serialize");
        assert!(json.contains("Error occurred"));
        assert!(json.contains("error_code"));
        assert!(json.contains("500"));
    }

    #[test]
    fn test_log_levels() {
        let info_log = StructuredLog::new(
            LogLevel::Info,
            "Info".to_string(),
            serde_json::json!({}),
        );
        let warn_log = StructuredLog::new(
            LogLevel::Warn,
            "Warning".to_string(),
            serde_json::json!({}),
        );
        let error_log = StructuredLog::new(
            LogLevel::Error,
            "Error".to_string(),
            serde_json::json!({}),
        );

        // Verify serialization includes level
        let info_json = serde_json::to_string(&info_log).unwrap();
        let warn_json = serde_json::to_string(&warn_log).unwrap();
        let error_json = serde_json::to_string(&error_log).unwrap();

        assert!(info_json.contains("info"));
        assert!(warn_json.contains("warn"));
        assert!(error_json.contains("error"));
    }
}
