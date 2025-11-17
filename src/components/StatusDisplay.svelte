<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { Submission, ProviderId } from '../types';
  import { SubmissionStatus } from '../types';

  // Props
  export let submissions: Submission[] = [];

  // T121: Listen for submission_status_changed events
  // TODO: Implement event listening when Tauri event system is integrated
  // For now, we'll poll for status updates

  // Get icon for provider
  function getProviderIcon(providerId: ProviderId): string {
    switch (providerId) {
      case 'ChatGPT':
        return '🤖';
      case 'Gemini':
        return '✨';
      case 'Claude':
        return '🧠';
      default:
        return '💬';
    }
  }

  // T122: Get status display info
  function getStatusInfo(status: SubmissionStatus): {
    label: string;
    color: string;
    icon: string;
  } {
    switch (status) {
      case SubmissionStatus.Pending:
        return { label: 'Pending', color: '#999', icon: '⏳' };
      case SubmissionStatus.InProgress:
        return { label: 'In Progress', color: '#4a9eff', icon: '🔄' };
      case SubmissionStatus.Retrying:
        return { label: 'Retrying', color: '#ff9800', icon: '🔁' };
      case SubmissionStatus.Success:
        return { label: 'Success', color: '#4caf50', icon: '✅' };
      case SubmissionStatus.Failed:
        return { label: 'Failed', color: '#f44336', icon: '❌' };
      default:
        return { label: 'Unknown', color: '#999', icon: '❓' };
    }
  }
</script>

<div class="status-display">
  {#if submissions.length === 0}
    <div class="empty-state">No submissions yet</div>
  {:else}
    <div class="submissions-list">
      <!-- T122: Display status for each provider -->
      {#each submissions as submission (submission.id)}
        {@const statusInfo = getStatusInfo(submission.status)}
        <div class="submission-item" data-testid={`submission-${submission.provider_id}`}>
          <div class="submission-header">
            <span class="provider-icon">{getProviderIcon(submission.provider_id)}</span>
            <span class="provider-name">{submission.provider_id}</span>
            <span class="status-badge" style="background-color: {statusInfo.color}">
              {statusInfo.icon}
            </span>
          </div>

          <!-- T123: Display error messages for failed submissions -->
          {#if submission.error_message}
            <div class="error-message">
              {submission.error_message}
            </div>
          {/if}

          <!-- T151: Loading spinner for in-progress submissions -->
          {#if submission.status === SubmissionStatus.InProgress || submission.status === SubmissionStatus.Retrying}
            <div class="loading-indicator">
              <div class="spinner"></div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .status-display {
    padding: 0.75rem;
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 6px;
    margin: 0.5rem 0;
    max-height: 150px;
    overflow-y: auto;
  }

  .empty-state {
    padding: 0.75rem;
    text-align: center;
    color: var(--text-secondary, #666);
    font-size: 0.85rem;
  }

  .submissions-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .submission-item {
    padding: 0.6rem;
    background: var(--bg-primary, white);
    border-radius: 4px;
    border: 1px solid var(--border-color, #ddd);
  }

  .submission-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .provider-icon {
    font-size: 1.2rem;
    line-height: 1;
  }

  .provider-name {
    flex: 1;
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-primary, #333);
  }

  .status-badge {
    padding: 0.2rem 0.4rem;
    border-radius: 50%;
    color: white;
    font-size: 0.8rem;
    line-height: 1;
  }

  .error-message {
    margin-top: 0.5rem;
    padding: 0.4rem;
    background: var(--error-bg, #fee);
    border-radius: 4px;
    color: var(--error-text, #c33);
    font-size: 0.8rem;
  }

  .loading-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 0.5rem;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid var(--border-color, #eee);
    border-top-color: var(--primary-color, #4a9eff);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
