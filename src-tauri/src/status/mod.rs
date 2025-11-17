// Status tracking module for prompt submissions
// Manages submission state machine and timeout/retry logic

use crate::types::{ProviderId, SubmissionErrorType, SubmissionStatus};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tracks a prompt submission to a specific provider
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Submission {
    pub id: String,
    pub provider_id: ProviderId,
    pub prompt_content: String,
    pub status: SubmissionStatus,
    pub attempt_count: u8,
    pub error_type: Option<SubmissionErrorType>,
    pub error_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

impl Submission {
    /// Creates a new submission in Pending state
    pub fn new(provider_id: ProviderId, prompt_content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            provider_id,
            prompt_content,
            status: SubmissionStatus::Pending,
            attempt_count: 0,
            error_type: None,
            error_message: None,
            started_at: None,
            completed_at: None,
        }
    }

    /// Transitions to InProgress state
    pub fn start(&mut self) -> Result<(), String> {
        if self.status != SubmissionStatus::Pending && self.status != SubmissionStatus::Retrying {
            return Err(format!(
                "Cannot start submission from {:?} state",
                self.status
            ));
        }

        self.status = SubmissionStatus::InProgress;
        self.attempt_count += 1;
        self.started_at = Some(current_timestamp());
        Ok(())
    }

    /// Transitions to Success state
    pub fn succeed(&mut self) -> Result<(), String> {
        if self.status != SubmissionStatus::InProgress {
            return Err(format!("Cannot succeed from {:?} state", self.status));
        }

        self.status = SubmissionStatus::Success;
        self.completed_at = Some(current_timestamp());
        Ok(())
    }

    /// Transitions to Failed or Retrying state based on error type
    pub fn fail(
        &mut self,
        error_type: SubmissionErrorType,
        error_message: String,
    ) -> Result<(), String> {
        if self.status != SubmissionStatus::InProgress && self.status != SubmissionStatus::Retrying
        {
            return Err(format!("Cannot fail from {:?} state", self.status));
        }

        // Check if error type should trigger retry
        if error_type.should_retry() && self.attempt_count < 2 {
            // Transition to Retrying
            self.status = SubmissionStatus::Retrying;
            self.error_type = Some(error_type);
            self.error_message = Some(error_message);
            Ok(())
        } else {
            // Transition to Failed
            self.status = SubmissionStatus::Failed;
            self.error_type = Some(error_type);
            self.error_message = Some(error_message);
            self.completed_at = Some(current_timestamp());
            Ok(())
        }
    }

    /// Checks if submission has exceeded timeout (30 seconds)
    pub fn is_timed_out(&self) -> bool {
        if let Some(started_at) = &self.started_at {
            if let Ok(elapsed) = time_since(started_at) {
                return elapsed > 30.0; // 30 second timeout
            }
        }
        false
    }
}

/// Returns current timestamp in ISO 8601 format
fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Calculates seconds since a timestamp
fn time_since(timestamp: &str) -> Result<f64, String> {
    use chrono::DateTime;

    let then =
        DateTime::parse_from_rfc3339(timestamp).map_err(|e| format!("Invalid timestamp: {}", e))?;

    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(then);

    Ok(duration.num_milliseconds() as f64 / 1000.0)
}

pub mod tracker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submission_creation() {
        let submission = Submission::new(ProviderId::ChatGPT, "Test prompt".to_string());

        assert_eq!(submission.provider_id, ProviderId::ChatGPT);
        assert_eq!(submission.prompt_content, "Test prompt");
        assert_eq!(submission.status, SubmissionStatus::Pending);
        assert_eq!(submission.attempt_count, 0);
        assert!(!submission.id.is_empty());
    }

    #[test]
    fn test_submission_start() {
        let mut submission = Submission::new(ProviderId::ChatGPT, "Test".to_string());

        let result = submission.start();
        assert!(result.is_ok());
        assert_eq!(submission.status, SubmissionStatus::InProgress);
        assert_eq!(submission.attempt_count, 1);
        assert!(submission.started_at.is_some());
    }

    #[test]
    fn test_submission_succeed() {
        let mut submission = Submission::new(ProviderId::ChatGPT, "Test".to_string());

        submission.start().unwrap();
        let result = submission.succeed();

        assert!(result.is_ok());
        assert_eq!(submission.status, SubmissionStatus::Success);
        assert!(submission.completed_at.is_some());
    }

    #[test]
    fn test_submission_fail_with_retry() {
        let mut submission = Submission::new(ProviderId::ChatGPT, "Test".to_string());

        submission.start().unwrap();
        let result = submission.fail(SubmissionErrorType::Timeout, "Timed out".to_string());

        assert!(result.is_ok());
        assert_eq!(submission.status, SubmissionStatus::Retrying);
        assert_eq!(submission.error_type, Some(SubmissionErrorType::Timeout));
    }

    #[test]
    fn test_submission_fail_no_retry() {
        let mut submission = Submission::new(ProviderId::ChatGPT, "Test".to_string());

        submission.start().unwrap();
        let result = submission.fail(
            SubmissionErrorType::AuthenticationError,
            "Not authenticated".to_string(),
        );

        assert!(result.is_ok());
        assert_eq!(submission.status, SubmissionStatus::Failed);
        assert!(submission.completed_at.is_some());
    }
}
