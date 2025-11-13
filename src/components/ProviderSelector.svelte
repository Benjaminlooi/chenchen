<script lang="ts">
  import { onMount } from 'svelte';
  import { tauri } from '../services/tauri';
  import type { Provider } from '../types';
  import { ProviderId } from '../types';

  // Component state
  let providers: Provider[] = [];
  let loading = true;
  let error: string | null = null;

  // Load providers on component mount
  onMount(async () => {
    try {
      providers = await tauri.getProviders();
      loading = false;
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load providers';
      loading = false;
    }
  });

  // Handle provider selection change
  async function handleProviderToggle(providerId: ProviderId, isSelected: boolean) {
    error = null;

    try {
      const updatedProvider = await tauri.updateProviderSelection(providerId, isSelected);

      // Update local state
      providers = providers.map((p) =>
        p.id === providerId ? updatedProvider : p
      );
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to update provider selection';

      // Revert the checkbox state on error
      providers = providers.map((p) =>
        p.id === providerId ? { ...p, is_selected: !isSelected } : p
      );
    }
  }

  // Get provider display information
  function getProviderIcon(providerId: ProviderId): string {
    switch (providerId) {
      case ProviderId.ChatGPT:
        return '🤖';
      case ProviderId.Gemini:
        return '✨';
      case ProviderId.Claude:
        return '🧠';
      default:
        return '💬';
    }
  }
</script>

<div class="provider-selector">
  <h2>Select LLM Providers</h2>

  {#if loading}
    <div class="loading">Loading providers...</div>
  {:else if error}
    <div class="error" role="alert">
      <strong>Error:</strong>
      {error}
    </div>
  {:else}
    <div class="provider-list">
      {#each providers as provider (provider.id)}
        <label class="provider-item">
          <input
            type="checkbox"
            checked={provider.is_selected}
            on:change={(e) =>
              handleProviderToggle(provider.id, e.currentTarget.checked)}
            data-testid={`provider-checkbox-${provider.id}`}
          />
          <span class="provider-icon">{getProviderIcon(provider.id)}</span>
          <span class="provider-name">{provider.name}</span>
          {#if !provider.is_authenticated}
            <span class="auth-badge" title="Login required">🔒</span>
          {/if}
        </label>
      {/each}
    </div>

    <div class="provider-info">
      <p>
        Selected: {providers.filter((p) => p.is_selected).length} / 3
      </p>
      <p class="hint">
        Select 1-3 providers to send your prompt to multiple LLMs simultaneously.
      </p>
    </div>
  {/if}
</div>

<style>
  .provider-selector {
    padding: 1rem;
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    margin: 1rem 0;
  }

  h2 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    color: var(--text-primary, #333);
  }

  .loading {
    text-align: center;
    padding: 2rem;
    color: var(--text-secondary, #666);
  }

  .error {
    padding: 0.75rem;
    background: var(--error-bg, #fee);
    border: 1px solid var(--error-border, #fcc);
    border-radius: 4px;
    color: var(--error-text, #c33);
  }

  .provider-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .provider-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: var(--bg-primary, white);
    border: 2px solid var(--border-color, #ddd);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .provider-item:hover {
    border-color: var(--primary-color, #4a9eff);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .provider-item input[type='checkbox'] {
    width: 20px;
    height: 20px;
    cursor: pointer;
  }

  .provider-icon {
    font-size: 1.5rem;
  }

  .provider-name {
    flex: 1;
    font-weight: 500;
    color: var(--text-primary, #333);
  }

  .auth-badge {
    font-size: 1rem;
    opacity: 0.6;
  }

  .provider-info {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color, #ddd);
  }

  .provider-info p {
    margin: 0.25rem 0;
    font-size: 0.9rem;
    color: var(--text-secondary, #666);
  }

  .hint {
    font-size: 0.85rem;
    font-style: italic;
  }
</style>
