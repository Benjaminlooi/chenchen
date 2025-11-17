// Tauri command invocation wrapper with type safety
// Provides strongly-typed wrappers around the Tauri invoke API

import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type { Provider, LayoutConfiguration, Submission } from '../types';
import { ProviderId } from '../types';

/**
 * Type-safe wrapper for Tauri command invocation
 */
class TauriService {
  /**
   * Gets all available providers
   */
  async getProviders(): Promise<Provider[]> {
    return tauriInvoke<Provider[]>('get_providers');
  }

  /**
   * Updates the selection state of a provider
   */
  async updateProviderSelection(
    providerId: ProviderId,
    isSelected: boolean
  ): Promise<Provider> {
    return tauriInvoke<Provider>('update_provider_selection', {
      providerId,
      isSelected,
    });
  }

  /**
   * Gets the layout configuration based on selected providers
   */
  async getLayoutConfiguration(): Promise<LayoutConfiguration> {
    return tauriInvoke<LayoutConfiguration>('get_layout_configuration');
  }

  /**
   * Submits a prompt to all selected providers
   * Returns array of created submissions with their IDs
   */
  async submitPrompt(prompt: string): Promise<Submission[]> {
    return tauriInvoke<Submission[]>('submit_prompt', { prompt });
  }

  /**
   * Gets the status of a specific submission
   */
  async getSubmissionStatus(submissionId: string): Promise<Submission> {
    return tauriInvoke<Submission>('get_submission_status', {
      submission_id: submissionId,
    });
  }
}

// Export singleton instance
export const tauri = new TauriService();
