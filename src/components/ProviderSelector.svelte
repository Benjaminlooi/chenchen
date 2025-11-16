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
      <label class="provider-item">
        <input
          type="checkbox"
          checked={provider.is_selected}
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
    border-radius: 4px;
    transition: background 0.2s;
  }

  .provider-item:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .provider-item input[type='checkbox'] {
    width: 16px;
    height: 16px;
    cursor: pointer;
  }

  .provider-icon {
    font-size: 1.2rem;
    line-height: 1;
  }

  .provider-name {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-primary, #333);
    white-space: nowrap;
  }
</style>
