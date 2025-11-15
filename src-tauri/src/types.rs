use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Provider identifier enum for the three supported LLM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ProviderId {
    ChatGPT,
    Gemini,
    Claude,
}

impl ProviderId {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderId::ChatGPT => "ChatGPT",
            ProviderId::Gemini => "Gemini",
            ProviderId::Claude => "Claude",
        }
    }

    pub fn url(&self) -> &'static str {
        match self {
            ProviderId::ChatGPT => "https://chat.openai.com/",
            ProviderId::Gemini => "https://gemini.google.com/",
            ProviderId::Claude => "https://claude.ai/",
        }
    }
}

/// Status of a prompt submission to a provider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum SubmissionStatus {
    Pending,
    InProgress,
    Retrying,
    Success,
    Failed,
}

/// Error types that can occur during prompt submission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum SubmissionErrorType {
    Timeout,
    NetworkError,
    AuthenticationError,
    RateLimitError,
    ElementNotFound,
    InjectionFailed,
}

impl SubmissionErrorType {
    /// Returns true if this error type should trigger a retry
    pub fn should_retry(&self) -> bool {
        matches!(
            self,
            SubmissionErrorType::Timeout | SubmissionErrorType::NetworkError
        )
    }
}

/// Standard error type for Tauri commands
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CommandError {
    pub code: String,
    pub message: String,
}

impl CommandError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::new("ValidationError", message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("NotFound", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("InternalError", message)
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for CommandError {}
