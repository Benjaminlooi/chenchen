// JavaScript injection module for submitting prompts to LLM providers
// Generates and executes JavaScript code to locate elements and trigger submissions

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Result of a JavaScript injection execution
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InjectionResult {
    pub success: bool,
    pub error_message: Option<String>,
    pub element_found: bool,
    pub submit_triggered: bool,
}

pub mod script_builder;
pub mod injector;
