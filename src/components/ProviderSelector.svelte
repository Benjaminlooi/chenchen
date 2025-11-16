<script lang="ts">
  import { onMount } from 'svelte';
  import { tauri } from '../services/tauri';
  import { focusProviderWebview } from '../services/providerWebviews';
  import type { Provider, AuthenticationStatus } from '../types';
  import { ProviderId } from '../types';

  // Component state
  let providers: Provider[] = [];
  let authStatuses: Map<ProviderId, AuthenticationStatus> = new Map();
  let loading = true;
  let togglingProviders = new Set<ProviderId>();

  // Load providers on component mount
  onMount(async () => {
    try {
      providers = await tauri.getProviders();
      loading = false;

      // Check authentication status for all providers
      await checkAllAuthStatuses();
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to load providers';
      loading = false;
    }
  });

  // Check authentication status for all providers
  async function checkAllAuthStatuses() {
    try {
      const statusPromises = providers.map((provider) =>
        tauri.checkAuthentication(provider.id).then((status) => ({
          providerId: provider.id,
          status,
        }))
      );

      const results = await Promise.all(statusPromises);

      // Update auth statuses map
      results.forEach(({ providerId, status }) => {
        authStatuses.set(providerId, status);
      });

      // Trigger reactivity
      authStatuses = new Map(authStatuses);
    } catch (e) {
      console.error('Failed to check authentication statuses:', e);
    }
  }

  // Handle provider selection change
  async function handleProviderToggle(providerId: ProviderId, isSelected: boolean) {
    // Mark as toggling for visual feedback
    togglingProviders.add(providerId);
    togglingProviders = togglingProviders;

    try {
      const updatedProvider = await tauri.updateProviderSelection(providerId, isSelected);

      // Update local state
      providers = providers.map((p) =>
        p.id === providerId ? updatedProvider : p
      );

      // Dispatch event to notify parent component of provider changes
      window.dispatchEvent(
        new CustomEvent('providers-changed', {
          detail: { providers },
        })
      );
    } catch (e) {
      // Revert the checkbox state on error
      providers = providers.map((p) =>
        p.id === providerId ? { ...p, is_selected: !isSelected } : p
      );
    } finally {
      // Remove toggling state
      togglingProviders.delete(providerId);
      togglingProviders = togglingProviders;
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
  {#if !loading}
    {#each providers as provider (provider.id)}
      <label
        class="provider-item"
        class:selected={provider.is_selected}
        class:toggling={togglingProviders.has(provider.id)}
      >
        <input
          type="checkbox"
          checked={provider.is_selected}
          disabled={togglingProviders.has(provider.id)}
          onchange={(e) =>
            handleProviderToggle(provider.id, e.currentTarget.checked)}
          data-testid={`provider-checkbox-${provider.id}`}
        />
        <span class="provider-icon">{getProviderIcon(provider.id)}</span>
        <span class="provider-name">{provider.name}</span>
      </label>
    {/each}
  {/if}
</div>

<style>
  .provider-selector {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .provider-item {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    cursor: pointer;
    padding: 0.3rem 0.6rem;
    border-radius: 6px;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    transform-origin: center;
    position: relative;
  }

  .provider-item:hover {
    background: rgba(0, 0, 0, 0.05);
    transform: scale(1.02);
  }

  .provider-item.selected {
    background: rgba(59, 130, 246, 0.1);
  }

  .provider-item.selected:hover {
    background: rgba(59, 130, 246, 0.15);
  }

  .provider-item.toggling {
    opacity: 0.6;
    cursor: wait;
    animation: pulse 1s ease-in-out infinite;
  }

  .provider-item.toggling:hover {
    transform: scale(1);
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 0.6;
    }
    50% {
      opacity: 0.8;
    }
  }

  .provider-item input[type='checkbox'] {
    width: 16px;
    height: 16px;
    cursor: pointer;
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .provider-item input[type='checkbox']:disabled {
    cursor: wait;
  }

  .provider-item input[type='checkbox']:active:not(:disabled) {
    transform: scale(0.9);
  }

  .provider-icon {
    font-size: 1.2rem;
    line-height: 1;
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .provider-item.selected .provider-icon {
    transform: scale(1.1);
  }

  .provider-name {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-primary, #333);
    white-space: nowrap;
    transition: color 0.2s ease;
  }

  .provider-item.selected .provider-name {
    color: rgb(59, 130, 246);
    font-weight: 600;
  }
</style>
