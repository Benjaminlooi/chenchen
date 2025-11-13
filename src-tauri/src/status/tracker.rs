// StatusTracker for managing submission states and emitting events

use super::Submission;
use crate::types::{CommandError, ProviderId, SubmissionErrorType};
use std::collections::HashMap;
use std::sync::Mutex;

/// Manages submission state and emits status change events
pub struct StatusTracker {
    submissions: Mutex<HashMap<String, Submission>>,
}

impl StatusTracker {
    /// Creates a new StatusTracker
    pub fn new() -> Self {
        Self {
            submissions: Mutex::new(HashMap::new()),
        }
    }

    /// Creates a new submission and stores it
    pub fn create_submission(
        &self,
        provider_id: ProviderId,
        prompt_content: String,
    ) -> Result<Submission, CommandError> {
        let submission = Submission::new(provider_id, prompt_content);
        let id = submission.id.clone();

        let mut submissions = self.submissions.lock().map_err(|e| {
            CommandError::internal(format!("Failed to acquire lock: {}", e))
        })?;

        submissions.insert(id, submission.clone());

        Ok(submission)
    }

    /// Gets the current status of a submission
    pub fn get_status(&self, submission_id: &str) -> Result<Submission, CommandError> {
        let submissions = self.submissions.lock().map_err(|e| {
            CommandError::internal(format!("Failed to acquire lock: {}", e))
        })?;

        submissions
            .get(submission_id)
            .cloned()
            .ok_or_else(|| {
                CommandError::not_found(format!("Submission not found: {}", submission_id))
            })
    }

    /// Updates submission status and emits event
    ///
    /// Note: Event emission will be implemented when we integrate with Tauri's event system
    pub fn update_status(
        &self,
        submission_id: &str,
        update_fn: impl FnOnce(&mut Submission) -> Result<(), String>,
    ) -> Result<Submission, CommandError> {
        let mut submissions = self.submissions.lock().map_err(|e| {
            CommandError::internal(format!("Failed to acquire lock: {}", e))
        })?;

        let submission = submissions
            .get_mut(submission_id)
            .ok_or_else(|| {
                CommandError::not_found(format!("Submission not found: {}", submission_id))
            })?;

        update_fn(submission).map_err(|e| {
            CommandError::internal(format!("Failed to update submission: {}", e))
        })?;

        // TODO: Emit submission_status_changed event
        // app.emit_all("submission_status_changed", submission.clone())?;

        Ok(submission.clone())
    }

    /// Starts a submission (Pending → InProgress)
    pub fn start_submission(&self, submission_id: &str) -> Result<Submission, CommandError> {
        self.update_status(submission_id, |s| s.start())
    }

    /// Marks a submission as successful
    pub fn succeed_submission(&self, submission_id: &str) -> Result<Submission, CommandError> {
        self.update_status(submission_id, |s| s.succeed())
    }

    /// Marks a submission as failed (with retry logic)
    pub fn fail_submission(
        &self,
        submission_id: &str,
        error_type: SubmissionErrorType,
        error_message: String,
    ) -> Result<Submission, CommandError> {
        self.update_status(submission_id, |s| s.fail(error_type, error_message))
    }

    /// Checks all in-progress submissions for timeouts
    pub fn check_timeouts(&self) -> Result<Vec<String>, CommandError> {
        let mut timed_out = Vec::new();

        let submissions = self.submissions.lock().map_err(|e| {
            CommandError::internal(format!("Failed to acquire lock: {}", e))
        })?;

        for (id, submission) in submissions.iter() {
            if submission.is_timed_out() {
                timed_out.push(id.clone());
            }
        }

        Ok(timed_out)
    }
}

impl Default for StatusTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_submission() {
        let tracker = StatusTracker::new();
        let submission = tracker
            .create_submission(ProviderId::ChatGPT, "Test".to_string())
            .unwrap();

        assert_eq!(submission.provider_id, ProviderId::ChatGPT);
        assert_eq!(submission.prompt_content, "Test");
    }

    #[test]
    fn test_get_status() {
        let tracker = StatusTracker::new();
        let submission = tracker
            .create_submission(ProviderId::ChatGPT, "Test".to_string())
            .unwrap();

        let retrieved = tracker.get_status(&submission.id).unwrap();
        assert_eq!(retrieved.id, submission.id);
    }

    #[test]
    fn test_start_submission() {
        let tracker = StatusTracker::new();
        let submission = tracker
            .create_submission(ProviderId::ChatGPT, "Test".to_string())
            .unwrap();

        let updated = tracker.start_submission(&submission.id).unwrap();
        assert_eq!(updated.attempt_count, 1);
    }

    #[test]
    fn test_succeed_submission() {
        let tracker = StatusTracker::new();
        let submission = tracker
            .create_submission(ProviderId::ChatGPT, "Test".to_string())
            .unwrap();

        tracker.start_submission(&submission.id).unwrap();
        let updated = tracker.succeed_submission(&submission.id).unwrap();

        assert_eq!(updated.status, crate::types::SubmissionStatus::Success);
    }

    #[test]
    fn test_fail_submission_with_retry() {
        let tracker = StatusTracker::new();
        let submission = tracker
            .create_submission(ProviderId::ChatGPT, "Test".to_string())
            .unwrap();

        tracker.start_submission(&submission.id).unwrap();
        let updated = tracker
            .fail_submission(
                &submission.id,
                SubmissionErrorType::Timeout,
                "Timed out".to_string(),
            )
            .unwrap();

        assert_eq!(updated.status, crate::types::SubmissionStatus::Retrying);
    }
}
