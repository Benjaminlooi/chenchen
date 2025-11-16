<script lang="ts">
  import { onMount } from 'svelte';
  import { tauri } from '../services/tauri';
  import type { Provider } from '../types';
  import { ProviderId } from '../types';

  // Component state
  let providers = $state<Provider[]>([]);
  let loading = $state(true);
  let togglingProviders = $state(new Set<ProviderId>());
  let isOpen = $state(false);
  let dropdownElement: HTMLDivElement | null = null;

  // Load providers on component mount
  onMount(() => {
    // Load providers
    (async () => {
      try {
        providers = await tauri.getProviders();
        loading = false;
      } catch (e) {
        console.error('Failed to load providers:', e);
        loading = false;
      }
    })();

    // Click outside handler
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownElement && !dropdownElement.contains(event.target as Node)) {
        isOpen = false;
      }
    };

    document.addEventListener('click', handleClickOutside);
    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });

  // Handle provider selection change
  async function handleProviderToggle(providerId: ProviderId, isSelected: boolean) {
    // Mark as toggling for visual feedback
    togglingProviders.add(providerId);
    togglingProviders = new Set(togglingProviders);

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
      togglingProviders = new Set(togglingProviders);
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

  // Get selected providers count and names
  function getSelectedInfo(): { count: number; names: string } {
    const selected = providers.filter(p => p.is_selected);
    const count = selected.length;
    const names = count === 0
      ? 'Select LLMs'
      : count === providers.length
      ? 'All LLMs'
      : selected.map(p => p.name).join(', ');
    return { count, names };
  }

  // Toggle dropdown
  function toggleDropdown() {
    isOpen = !isOpen;
  }
</script>

<div class="provider-selector" bind:this={dropdownElement}>
  {#if !loading}
    {@const selectedInfo = getSelectedInfo()}

    <!-- Dropdown Button -->
    <button
      class="dropdown-button"
      class:open={isOpen}
      onclick={toggleDropdown}
      aria-label="Select LLM providers"
    >
      <span class="button-label">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M9 3H15M3 8H21M5 8H19V18C19 18.5304 18.7893 19.0391 18.4142 19.4142C18.0391 19.7893 17.5304 20 17 20H7C6.46957 20 5.96086 19.7893 5.58579 19.4142C5.21071 19.0391 5 18.5304 5 18V8Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span class="label-text">{selectedInfo.names}</span>
        {#if selectedInfo.count > 0}
          <span class="count-badge">{selectedInfo.count}</span>
        {/if}
      </span>
      <svg class="chevron" class:rotated={isOpen} viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M6 9L12 15L18 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>

    <!-- Dropdown Menu -->
    {#if isOpen}
      <div class="dropdown-menu">
        {#each providers as provider (provider.id)}
          <label
            class="menu-item"
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
            <span class="checkbox-custom"></span>
            <span class="provider-icon">{getProviderIcon(provider.id)}</span>
            <span class="provider-name">{provider.name}</span>
          </label>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .provider-selector {
    position: relative;
  }

  .dropdown-button {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    font-family: inherit;
    font-size: 0.9rem;
    color: #333;
    min-width: 180px;
  }

  .dropdown-button:hover {
    background: rgba(255, 255, 255, 0.95);
    border-color: rgba(102, 126, 234, 0.3);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  }

  .dropdown-button.open {
    background: rgba(255, 255, 255, 0.95);
    border-color: rgba(102, 126, 234, 0.5);
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  .button-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
  }

  .button-label svg {
    width: 18px;
    height: 18px;
    color: #667eea;
  }

  .label-text {
    flex: 1;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: 500;
  }

  .count-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 20px;
    height: 20px;
    padding: 0 6px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border-radius: 10px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .chevron {
    width: 16px;
    height: 16px;
    transition: transform 0.2s ease;
    flex-shrink: 0;
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .dropdown-menu {
    position: absolute;
    bottom: calc(100% + 4px);
    left: 0;
    min-width: 100%;
    max-height: 200px;
    overflow-y: auto;
    background: rgba(255, 255, 255, 0.98);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 8px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.12);
    padding: 0.25rem;
    z-index: 10000;
    animation: dropdown-slide-up 0.2s ease;
  }

  /* Scrollbar styling for dropdown */
  .dropdown-menu::-webkit-scrollbar {
    width: 6px;
  }

  .dropdown-menu::-webkit-scrollbar-track {
    background: transparent;
  }

  .dropdown-menu::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 3px;
  }

  .dropdown-menu::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
  }

  @keyframes dropdown-slide-up {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.65rem 0.75rem;
    cursor: pointer;
    border-radius: 6px;
    transition: all 0.15s ease;
    position: relative;
  }

  .menu-item:hover {
    background: rgba(102, 126, 234, 0.08);
  }

  .menu-item.selected {
    background: rgba(102, 126, 234, 0.12);
  }

  .menu-item.toggling {
    opacity: 0.5;
    cursor: wait;
  }

  .menu-item input[type='checkbox'] {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .checkbox-custom {
    width: 18px;
    height: 18px;
    border: 2px solid #ccc;
    border-radius: 4px;
    transition: all 0.2s ease;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .menu-item:hover .checkbox-custom {
    border-color: #667eea;
  }

  .menu-item.selected .checkbox-custom {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-color: #667eea;
  }

  .menu-item.selected .checkbox-custom::after {
    content: '';
    width: 5px;
    height: 9px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }

  .provider-icon {
    font-size: 1.2rem;
    line-height: 1;
  }

  .provider-name {
    font-size: 0.9rem;
    font-weight: 500;
    color: #333;
    white-space: nowrap;
  }

  .menu-item.selected .provider-name {
    color: #667eea;
    font-weight: 600;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .dropdown-button {
      background: rgba(30, 30, 30, 0.8);
      border-color: rgba(255, 255, 255, 0.1);
      color: #f6f6f6;
    }

    .dropdown-button:hover,
    .dropdown-button.open {
      background: rgba(30, 30, 30, 0.95);
      border-color: rgba(102, 126, 234, 0.5);
    }

    .dropdown-menu {
      background: rgba(30, 30, 30, 0.98);
      border-color: rgba(255, 255, 255, 0.1);
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
    }

    .dropdown-menu::-webkit-scrollbar-thumb {
      background: rgba(255, 255, 255, 0.2);
    }

    .dropdown-menu::-webkit-scrollbar-thumb:hover {
      background: rgba(255, 255, 255, 0.3);
    }

    .menu-item:hover {
      background: rgba(102, 126, 234, 0.15);
    }

    .menu-item.selected {
      background: rgba(102, 126, 234, 0.2);
    }

    .checkbox-custom {
      border-color: #666;
    }

    .provider-name {
      color: #f6f6f6;
    }

    .menu-item.selected .provider-name {
      color: #a78bfa;
    }
  }
</style>
