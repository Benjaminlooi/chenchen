<script lang="ts">
  import { fade } from 'svelte/transition';
  import type { Submission } from '../types';
  import { SubmissionStatus } from '../types';

  // Props using Svelte 5 runes
  interface Props {
    submissions: Submission[];
    onClose?: () => void;
  }
  let { submissions = [], onClose }: Props = $props();

  // State
  let isCollapsed = $state(false);

  // Derived state (Svelte 5 Runes)
  let totalSubmissions = $derived(submissions.length);
  let completedSubmissions = $derived(
    submissions.filter(
      (s) => s.status === SubmissionStatus.Success || s.status === SubmissionStatus.Failed
    ).length
  );
  let succeededCount = $derived(submissions.filter((s) => s.status === SubmissionStatus.Success).length);
  let progressPercent = $derived(
    totalSubmissions > 0 ? (completedSubmissions / totalSubmissions) * 100 : 0
  );
  let allFinished = $derived(totalSubmissions > 0 && completedSubmissions === totalSubmissions);

  // Helpers
  function getProviderColor(providerId: string): string {
    switch (providerId) {
      case 'ChatGPT': return 'hsl(163, 75%, 45%)';
      case 'Gemini': return 'hsl(217, 90%, 60%)';
      case 'Claude': return 'hsl(14, 75%, 60%)';
      case 'Perplexity': return 'hsl(172, 75%, 45%)';
      case 'DeepSeek': return 'hsl(228, 95%, 65%)';
      default: return 'hsl(220, 10%, 60%)';
    }
  }

  function getStatusLabel(status: SubmissionStatus, attempt: number): string {
    switch (status) {
      case SubmissionStatus.Pending: return 'Pending...';
      case SubmissionStatus.InProgress: return 'Broadcasting...';
      case SubmissionStatus.Retrying: return `Retrying (Attempt ${attempt}/2)`;
      case SubmissionStatus.Success: return 'Delivered';
      case SubmissionStatus.Failed: return 'Failed';
      default: return 'Idle';
    }
  }
</script>

{#if totalSubmissions > 0}
  <div 
    class="status-display-hud glass-strong" 
    class:collapsed={isCollapsed}
    role="region" 
    aria-label="Submission status console"
  >
    <!-- Top progress bar indicator -->
    <div class="overall-progress-bar" style:width="{progressPercent}%" style:background-color={allFinished ? 'var(--success-color)' : 'var(--primary-color)'}></div>

    <!-- HUD Header Panel -->
    <div 
      class="hud-header" 
      role="button" 
      tabindex="0" 
      onclick={() => isCollapsed = !isCollapsed}
      onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); isCollapsed = !isCollapsed; } }}
    >
      <div class="header-main">
        <div class="hud-title">
          <div class="pulse-indicator" style="background: {allFinished ? 'var(--success-color)' : 'var(--primary-color)'};"></div>
          <span>Submission Console</span>
        </div>
        <span class="hud-status">
          {#if allFinished}
            Ready ({succeededCount}/{totalSubmissions} success)
          {:else}
            Processing ({completedSubmissions}/{totalSubmissions})
          {/if}
        </span>
      </div>

      <!-- Action buttons -->
      <div class="header-actions">
        <button 
          class="icon-button" 
          onclick={(e) => { e.stopPropagation(); isCollapsed = !isCollapsed; }}
          title={isCollapsed ? "Expand HUD" : "Collapse HUD"}
          aria-label={isCollapsed ? "Expand HUD" : "Collapse HUD"}
        >
          <svg class="chevron" class:rotated={!isCollapsed} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M18 15L12 9L6 15" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
        {#if allFinished && onClose}
          <button 
            class="icon-button close" 
            onclick={(e) => { e.stopPropagation(); onClose(); }}
            title="Dismiss Console"
            aria-label="Dismiss Console"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        {/if}
      </div>
    </div>

    <!-- HUD details container -->
    <div class="hud-content">
      <div class="submissions-list">
        {#each submissions as submission (submission.id)}
          {@const color = getProviderColor(submission.provider_id)}
          <div class="submission-row" style:--theme-color={color} data-testid={`submission-${submission.provider_id}`}>
            <div class="row-header">
              <!-- Dot Indicator -->
              <span class="provider-badge" style:background-color={color}>
                {submission.provider_id.substring(0, 1).toUpperCase()}
              </span>
              <span class="provider-name">{submission.provider_id}</span>
              
              <!-- Loader Status -->
              <div class="status-indicator-tag">
                {#if submission.status === SubmissionStatus.InProgress || submission.status === SubmissionStatus.Retrying}
                  <svg class="row-spinner" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="32" class="spinner-circle"></circle>
                  </svg>
                {:else if submission.status === SubmissionStatus.Success}
                  <div class="status-badge-circle success">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                      <path d="M20 6L9 17L4 12" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </div>
                {:else if submission.status === SubmissionStatus.Failed}
                  <div class="status-badge-circle failed">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                      <path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                  </div>
                {:else}
                  <div class="status-badge-circle pending"></div>
                {/if}
                <span class="status-text" class:success={submission.status === SubmissionStatus.Success} class:failed={submission.status === SubmissionStatus.Failed}>
                  {getStatusLabel(submission.status, submission.attempt_count)}
                </span>
              </div>
            </div>

            <!-- Error Banner -->
            {#if submission.error_message}
              <div class="error-panel" transition:fade={{ duration: 150 }}>
                <span class="error-title">Error Code: {submission.error_type || 'Unknown'}</span>
                <span class="error-desc">{submission.error_message}</span>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  /* Embedded console framework */
  .status-display-hud {
    position: relative;
    width: 100%;
    border-radius: var(--radius-lg);
    z-index: 1;
    overflow: hidden;
    box-shadow: var(--shadow-lg), 0 8px 32px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    max-height: 100%;
    transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
    animation: dropdown-enter 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    transform-origin: top right;
  }

  .status-display-hud.collapsed {
    max-height: 52px;
  }

  .status-display-hud.collapsed .hud-content {
    opacity: 0;
    pointer-events: none;
  }

  /* Overall linear progress bar */
  .overall-progress-bar {
    position: absolute;
    top: 0;
    left: 0;
    height: 3px;
    width: 0;
    transition: width 0.4s cubic-bezier(0.16, 1, 0.3, 1), background var(--transition-normal);
    z-index: 50;
  }

  /* HUD Header */
  .hud-header {
    padding: 0.85rem 1.2rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: hsla(220, 18%, 6%, 0.45);
    border-bottom: 1px solid var(--border-color);
    cursor: pointer;
    user-select: none;
    z-index: 2;
  }

  .header-main {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    min-width: 0;
  }

  .hud-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-family: var(--font-heading);
    font-size: 0.82rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    color: var(--text-primary);
  }

  .hud-status {
    font-size: 0.72rem;
    color: var(--text-secondary);
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }

  /* Icon Buttons */
  .icon-button {
    background: transparent;
    border: none;
    width: 26px;
    height: 26px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all var(--transition-normal);
    box-shadow: none;
    padding: 0;
  }

  .icon-button:hover {
    color: var(--text-primary);
    background: hsla(220, 20%, 100%, 0.05);
  }

  .icon-button svg {
    width: 14px;
    height: 14px;
  }

  .chevron {
    transform: rotate(180deg);
    transition: transform var(--transition-spring);
  }

  .chevron.rotated {
    transform: rotate(0deg);
  }

  .icon-button.close:hover {
    color: var(--error-color);
    background: var(--error-bg);
  }

  /* HUD Content */
  .hud-content {
    flex: 1;
    overflow-y: auto;
    padding: 0.65rem;
    background: hsla(220, 20%, 3%, 0.35);
    transition: opacity 0.3s ease;
  }

  .submissions-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  /* Row Item */
  .submission-row {
    padding: 0.65rem 0.85rem;
    border-radius: var(--radius-md);
    background: hsla(220, 15%, 8%, 0.6);
    border: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    transition: border-color var(--transition-normal);
  }

  .submission-row:hover {
    border-color: hsla(from var(--theme-color) h s l / 0.2);
  }

  .row-header {
    display: flex;
    align-items: center;
    gap: 0.65rem;
  }

  /* Badge indicator */
  .provider-badge {
    width: 22px;
    height: 22px;
    border-radius: 6px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 0.72rem;
    font-weight: 700;
    box-shadow: var(--shadow-sm);
    flex-shrink: 0;
  }

  .provider-name {
    flex: 1;
    font-weight: 600;
    font-size: 0.88rem;
    color: var(--text-primary);
  }

  /* Status tag inline indicator */
  .status-indicator-tag {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }

  .status-badge-circle {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .status-badge-circle.success {
    background: var(--success-bg);
    color: var(--success-color);
    border: 1px solid hsla(150, 80%, 45%, 0.2);
  }

  .status-badge-circle.failed {
    background: var(--error-bg);
    color: var(--error-color);
    border: 1px solid hsla(355, 85%, 55%, 0.2);
  }

  .status-badge-circle.pending {
    background: hsla(220, 20%, 100%, 0.05);
    border: 1px solid var(--border-color);
  }

  .status-badge-circle svg {
    width: 9px;
    height: 9px;
  }

  .status-text {
    font-size: 0.72rem;
    font-weight: 500;
    color: var(--text-tertiary);
  }

  .status-text.success {
    color: var(--success-color);
    font-weight: 600;
  }

  .status-text.failed {
    color: var(--error-color);
    font-weight: 600;
  }

  /* Inline sub-spinner */
  .row-spinner {
    width: 14px;
    height: 14px;
    animation: spin-pulsing 1s linear infinite;
    color: var(--primary-color);
  }

  /* Error overlay inside rows */
  .error-panel {
    padding: 0.45rem 0.65rem;
    background: hsla(355, 85%, 55%, 0.04);
    border: 1px solid hsla(355, 85%, 55%, 0.15);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
  }

  .error-panel .error-title {
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--error-color);
    text-transform: uppercase;
    letter-spacing: 0.02em;
  }

  .error-panel .error-desc {
    font-size: 0.72rem;
    color: hsl(355, 95%, 85%);
    line-height: 1.35;
  }
</style>
