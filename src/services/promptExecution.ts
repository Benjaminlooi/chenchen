import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import type { ProviderId } from '../types';
import { activeWebviews } from './providerWebviews';

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
 * This listens for execute-prompt events from the backend and executes scripts in provider webviews
 */
export async function initPromptExecutionListener(): Promise<void> {
  // Clean up existing listener if any
  if (unlistenExecutePrompt) {
    unlistenExecutePrompt();
  }

  // Listen for execute-prompt events from the backend
  unlistenExecutePrompt = await listen<ExecutePromptPayload>('execute-prompt', async (event) => {
    const { submission_id, provider_id, script } = event.payload;

    console.log(`[PromptExecution] Received execute-prompt event for ${provider_id}`, {
      submission_id,
      script_length: script.length,
    });

    // Get the webview for this provider
    const activeWebview = activeWebviews.get(provider_id);

    if (!activeWebview) {
      console.error(`[PromptExecution] No webview found for provider ${provider_id}`);

      // Report failure to backend
      await reportExecutionResult({
        submission_id,
        provider_id,
        success: false,
        error_message: `No webview found for provider ${provider_id}`,
        element_found: false,
        submit_triggered: false,
      });
      return;
    }

    try {
      // Execute the script in the webview
      console.log(`[PromptExecution] Executing script in ${provider_id} webview`);
      console.log(`[PromptExecution] Script preview:`, script.substring(0, 200) + '...');

      await activeWebview.webview.eval(script);

      // If eval() doesn't throw, assume success
      // Check the provider webview console for detailed [ChenChen] logs
      console.log(`[PromptExecution] Script executed without errors in ${provider_id}`);

      // Report success to backend
      await reportExecutionResult({
        submission_id,
        provider_id,
        success: true,
        element_found: true,
        submit_triggered: true,
      });
    } catch (error) {
      console.error(`[PromptExecution] Script execution threw error in ${provider_id}:`, error);
      console.error(`[PromptExecution] Check the ${provider_id} webview console for [ChenChen] error logs`);

      // Report failure to backend
      await reportExecutionResult({
        submission_id,
        provider_id,
        success: false,
        error_message: error instanceof Error ? error.message : String(error),
        element_found: false,
        submit_triggered: false,
      });
    }
  });

  console.log('[PromptExecution] Listener initialized');
}

/**
 * Report execution result back to the backend
 */
async function reportExecutionResult(payload: ExecutionResultPayload): Promise<void> {
  try {
    await invoke('report_execution_result', { payload });
    console.log('[PromptExecution] Reported result to backend', {
      submission_id: payload.submission_id,
      success: payload.success,
    });
  } catch (error) {
    console.error('[PromptExecution] Failed to report result to backend:', error);
  }
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
