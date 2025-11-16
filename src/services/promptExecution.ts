import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import type { ProviderId } from '../types';

interface ExecutePromptPayload {
  submission_id: string;
  provider_id: ProviderId;
  script: string;
}

interface ExecutionResultPayload {
  submission_id: string;
  provider_id: ProviderId;
  success: boolean;
  error_message?: string;
  element_found: boolean;
  submit_triggered: boolean;
}

let unlistenExecutePrompt: UnlistenFn | null = null;

/**
 * Initialize the prompt execution listener
 * This listens for execute-prompt events from the backend
 * The backend will execute scripts directly in webviews it created
 */
export async function initPromptExecutionListener(): Promise<void> {
  console.log('[PromptExecution] Listener initialized (backend handles execution)');
  // Note: With Rust-created webviews, the backend executes scripts directly
  // We don't need a frontend listener anymore, but keep this for potential future use
}

/**
 * Clean up the prompt execution listener
 */
export function cleanupPromptExecutionListener(): void {
  if (unlistenExecutePrompt) {
    unlistenExecutePrompt();
    unlistenExecutePrompt = null;
    console.log('[PromptExecution] Listener cleaned up');
  }
}
