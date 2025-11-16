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
  let error: string | null = null;
  let checkingAuth = false;
  let openingWebview: ProviderId | null = null;
  let isDropdownOpen = false;

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
    checkingAuth = true;

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
    } finally {
      checkingAuth = false;
    }
  }

  // Handle provider selection change
  async function handleProviderToggle(providerId: ProviderId, isSelected: boolean) {
    error = null;

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
      error = e instanceof Error ? e.message : 'Failed to update provider selection';

      // Revert the checkbox state on error
      providers = providers.map((p) =>
        p.id === providerId ? { ...p, is_selected: !isSelected } : p
      );
    }
  }

  // Open provider login page
  async function handleLoginClick(providerId: ProviderId) {
    error = null;
    openingWebview = providerId;

    try {
      const provider = providers.find((p) => p.id === providerId);
      if (provider && !provider.is_selected) {
        await handleProviderToggle(providerId, true);
      }

      await focusProviderWebview(providerId);
    } catch (e) {
      console.error('Failed to focus provider webview:', e);
      error = e instanceof Error ? e.message : 'Failed to focus provider webview';
    } finally {
      if (openingWebview === providerId) {
        openingWebview = null;
      }
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

  // Toggle dropdown open/close
  function toggleDropdown() {
    isDropdownOpen = !isDropdownOpen;
  }

  // Close dropdown when clicking outside
  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.provider-selector')) {
      isDropdownOpen = false;
    }
  }

  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });
</script>

<div class="provider-selector">
  {#if loading}
    <div class="dropdown-header loading">Loading providers...</div>
  {:else if error}
    <div class="error" role="alert">
      <strong>Error:</strong>
      {error}
    </div>
  {:else}
    <button class="dropdown-header" onclick={toggleDropdown} type="button">
      <span class="header-content">
        <span class="header-icons">
          {#each providers.filter((p) => p.is_selected) as provider}
            <span class="provider-icon-small" title={provider.name}>
              {getProviderIcon(provider.id)}
            </span>
          {/each}
        </span>
        <span class="header-text">
          Providers ({providers.filter((p) => p.is_selected).length}/3)
        </span>
      </span>
      <span class="dropdown-arrow" class:open={isDropdownOpen}>▼</span>
    </button>

    {#if isDropdownOpen}
      <div class="dropdown-menu">
        <div class="provider-list">
          {#each providers as provider (provider.id)}
            <div class="provider-item">
              <label class="provider-checkbox-label">
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

              {#if checkingAuth}
                <span class="auth-checking">Checking...</span>
              {:else}
                {@const authStatus = authStatuses.get(provider.id)}
                {#if authStatus && authStatus.requires_login}
                  <button
                    class="login-button"
                    onclick={() => handleLoginClick(provider.id)}
                    title="Login required to use this provider"
                    disabled={openingWebview === provider.id}
                  >
                    {#if openingWebview === provider.id}
                      Opening...
                    {:else}
                      🔒 Login
                    {/if}
                  </button>
                {:else if authStatus && authStatus.is_authenticated}
                  <span class="auth-badge auth-ok" title="Authenticated">✓</span>
                {/if}
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .provider-selector {
    position: relative;
    width: 100%;
  }

  .dropdown-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0.5rem 0.75rem;
    background: var(--bg-secondary, #f5f5f5);
    border: 2px solid var(--border-color, #ddd);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.9rem;
    font-family: inherit;
  }

  .dropdown-header:not(.loading):hover {
    border-color: var(--primary-color, #4a9eff);
    background: var(--bg-primary, white);
  }

  .dropdown-header.loading {
    cursor: default;
    color: var(--text-secondary, #666);
  }

  .header-content {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .header-icons {
    display: flex;
    gap: 0.25rem;
  }

  .provider-icon-small {
    font-size: 1.2rem;
    line-height: 1;
  }

  .header-text {
    font-weight: 500;
    color: var(--text-primary, #333);
  }

  .dropdown-arrow {
    font-size: 0.75rem;
    color: var(--text-secondary, #666);
    transition: transform 0.2s;
  }

  .dropdown-arrow.open {
    transform: rotate(180deg);
  }

  .dropdown-menu {
    position: absolute;
    top: calc(100% + 0.25rem);
    left: 0;
    right: 0;
    background: var(--bg-primary, white);
    border: 2px solid var(--border-color, #ddd);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 1000;
    max-height: 300px;
    overflow-y: auto;
  }

  .provider-list {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .provider-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.6rem 0.75rem;
    border-bottom: 1px solid var(--border-color, #ddd);
    transition: background 0.2s;
  }

  .provider-item:last-child {
    border-bottom: none;
  }

  .provider-item:hover {
    background: var(--bg-secondary, #f5f5f5);
  }

  .provider-checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    cursor: pointer;
  }

  .provider-checkbox-label input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .provider-icon {
    font-size: 1.3rem;
    line-height: 1;
  }

  .provider-name {
    flex: 1;
    font-weight: 500;
    color: var(--text-primary, #333);
    font-size: 0.9rem;
  }

  .auth-checking {
    font-size: 0.8rem;
    color: var(--text-secondary, #999);
    font-style: italic;
  }

  .auth-badge {
    font-size: 1rem;
    opacity: 0.6;
  }

  .auth-badge.auth-ok {
    color: var(--success-color, #4caf50);
    opacity: 1;
    font-weight: bold;
  }

  .login-button {
    padding: 0.3rem 0.6rem;
    font-size: 0.75rem;
    background: var(--warning-bg, #fff3cd);
    color: var(--warning-text, #856404);
    border: 1px solid var(--warning-border, #ffc107);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .login-button:hover {
    background: var(--warning-hover, #ffc107);
    color: var(--warning-hover-text, #000);
  }

  .login-button:active {
    transform: scale(0.98);
  }

  .error {
    padding: 0.5rem;
    background: var(--error-bg, #fee);
    border: 1px solid var(--error-border, #fcc);
    border-radius: 4px;
    color: var(--error-text, #c33);
    font-size: 0.85rem;
  }
</style>
