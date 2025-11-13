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
  <h2>Submission Status</h2>

  {#if submissions.length === 0}
    <div class="empty-state">
      <p>No submissions yet. Enter a prompt and click "Send to Selected Providers" to get started.</p>
    </div>
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
              {statusInfo.icon} {statusInfo.label}
            </span>
          </div>

          <div class="submission-details">
            <div class="detail-item">
              <span class="detail-label">Attempts:</span>
              <span class="detail-value">{submission.attempt_count}/2</span>
            </div>

            {#if submission.started_at}
              <div class="detail-item">
                <span class="detail-label">Started:</span>
                <span class="detail-value">{new Date(submission.started_at).toLocaleTimeString()}</span>
              </div>
            {/if}

            {#if submission.completed_at}
              <div class="detail-item">
                <span class="detail-label">Completed:</span>
                <span class="detail-value">{new Date(submission.completed_at).toLocaleTimeString()}</span>
              </div>
            {/if}
          </div>

          <!-- T123: Display error messages for failed submissions -->
          {#if submission.error_message}
            <div class="error-message">
              <strong>Error:</strong> {submission.error_message}
              {#if submission.error_type}
                <span class="error-type">({submission.error_type})</span>
              {/if}
            </div>
          {/if}

          <!-- T151: Loading spinner for in-progress submissions -->
          {#if submission.status === SubmissionStatus.InProgress || submission.status === SubmissionStatus.Retrying}
            <div class="loading-indicator">
              <div class="spinner"></div>
              <span>Processing...</span>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .status-display {
    padding: 1.5rem;
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    margin: 1rem 0;
  }

  h2 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    color: var(--text-primary, #333);
  }

  .empty-state {
    padding: 2rem;
    text-align: center;
    color: var(--text-secondary, #666);
  }

  .submissions-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .submission-item {
    padding: 1rem;
    background: var(--bg-primary, white);
    border-radius: 6px;
    border: 2px solid var(--border-color, #ddd);
  }

  .submission-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .provider-icon {
    font-size: 1.5rem;
  }

  .provider-name {
    flex: 1;
    font-weight: 600;
    font-size: 1.1rem;
    color: var(--text-primary, #333);
  }

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    color: white;
    font-size: 0.85rem;
    font-weight: 600;
  }

  .submission-details {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    padding: 0.75rem 0;
    border-top: 1px solid var(--border-color, #eee);
    border-bottom: 1px solid var(--border-color, #eee);
  }

  .detail-item {
    display: flex;
    gap: 0.5rem;
  }

  .detail-label {
    color: var(--text-secondary, #666);
    font-size: 0.9rem;
  }

  .detail-value {
    color: var(--text-primary, #333);
    font-size: 0.9rem;
    font-weight: 500;
  }

  .error-message {
    margin-top: 0.75rem;
    padding: 0.75rem;
    background: var(--error-bg, #fee);
    border-radius: 4px;
    color: var(--error-text, #c33);
    font-size: 0.9rem;
  }

  .error-type {
    color: var(--text-secondary, #999);
    font-size: 0.85rem;
  }

  .loading-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.75rem;
    padding: 0.5rem;
    color: var(--primary-color, #4a9eff);
    font-size: 0.9rem;
  }

  .spinner {
    width: 16px;
    height: 16px;
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
