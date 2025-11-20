<script lang="ts">
  import { onMount } from 'svelte';
  import { Menu, CheckMenuItem } from '@tauri-apps/api/menu';
  import { LogicalPosition } from '@tauri-apps/api/window';
  import { tauri } from '../services/tauri';
  import type { Provider } from '../types';
  import { ProviderId } from '../types';

  // Component state
  let providers = $state<Provider[]>([]);
  let loading = $state(true);
  let togglingProviders = $state(new Set<ProviderId>());
  let buttonElement: HTMLButtonElement | null = null;

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

  // Show native popup menu
  async function showNativeMenu() {
    if (!buttonElement) return;

    try {
      // Create menu
      const menu = await Menu.new();

      // Add menu items for each provider
      for (const provider of providers) {
        const menuItem = await CheckMenuItem.new({
          text: `${getProviderIcon(provider.id)} ${provider.name}`,
          checked: provider.is_selected,
          enabled: !togglingProviders.has(provider.id),
          action: async () => {
            // Toggle provider selection
            await handleProviderToggle(provider.id, !provider.is_selected);
          }
        });
        await menu.append(menuItem);
      }

      // Get button position
      const rect = buttonElement.getBoundingClientRect();

      // Show menu at button position
      await menu.popup(new LogicalPosition(rect.left, rect.bottom + 4));
    } catch (error) {
      console.error('Failed to show native menu:', error);
    }
  }
</script>

<div class="provider-selector">
  {#if !loading}
    {@const selectedInfo = getSelectedInfo()}

    <!-- Native Menu Button -->
    <button
      bind:this={buttonElement}
      class="menu-button"
      onclick={showNativeMenu}
      aria-label="Select LLM providers"
      data-testid="provider-selector-button"
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
      <svg class="chevron" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M6 9L12 15L18 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
  {/if}
</div>

<style>
  .provider-selector {
    position: relative;
  }

  .menu-button {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem 0.5rem 1.25rem;
    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 9999px; /* Pill shape */
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    font-family: 'Inter', sans-serif;
    font-size: 0.95rem;
    color: #ffffff;
    min-width: 180px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  }

  .menu-button:hover {
    background: rgba(40, 40, 40, 0.8);
    border-color: rgba(255, 255, 255, 0.2);
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.3), 0 0 0 2px rgba(59, 130, 246, 0.3);
    transform: translateY(-2px);
  }

  .menu-button:active {
    transform: scale(0.98);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  }

  .button-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
  }

  .button-label svg {
    width: 20px;
    height: 20px;
    color: #60a5fa; /* Light blue */
    transition: color 0.3s ease;
  }

  .menu-button:hover .button-label svg {
    color: #93c5fd;
  }

  .label-text {
    flex: 1;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: 500;
    letter-spacing: 0.01em;
  }

  .count-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 22px;
    height: 22px;
    padding: 0 6px;
    background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
    color: white;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    box-shadow: 0 2px 5px rgba(37, 99, 235, 0.4);
  }

  .chevron {
    width: 16px;
    height: 16px;
    color: rgba(255, 255, 255, 0.5);
    transition: transform 0.3s ease, color 0.3s ease;
    flex-shrink: 0;
  }

  .menu-button:hover .chevron {
    color: rgba(255, 255, 255, 0.8);
    transform: translateY(1px);
  }
</style>
