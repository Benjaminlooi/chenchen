<script lang="ts">
  import { onMount } from 'svelte';
  import { tauri } from '../services/tauri';
  import type { Provider } from '../types';
  import { ProviderId } from '../types';

  interface Props {
    ondropdowntoggled?: (open: boolean, buttonRect: DOMRect | null) => void;
  }
  let { ondropdowntoggled }: Props = $props();

  // State
  let providers = $state<Provider[]>([]);
  let loading = $state(true);
  let togglingProviders = $state(new Set<ProviderId>());
  let dropdownOpen = $state(false);
  let buttonElement = $state<HTMLButtonElement | null>(null);
  let dropdownElement = $state<HTMLDivElement | null>(null);
  let activeIndex = $state(-1); // Keyboard navigation index

  // Load providers on component mount
  onMount(() => {
    (async () => {
      try {
        providers = await tauri.getProviders();
        loading = false;
      } catch (e) {
        console.error('Failed to load providers:', e);
        loading = false;
      }
    })();

    // Close dropdown on click outside
    function handleClickOutside(event: MouseEvent) {
      if (
        dropdownOpen &&
        buttonElement &&
        !buttonElement.contains(event.target as Node) &&
        dropdownElement &&
        !dropdownElement.contains(event.target as Node)
      ) {
        dropdownOpen = false;
        ondropdowntoggled?.(false, null);
      }
    }

    window.addEventListener('mousedown', handleClickOutside);
    return () => {
      window.removeEventListener('mousedown', handleClickOutside);
    };
  });

  // Handle provider selection change
  async function handleProviderToggle(providerId: ProviderId, isSelected: boolean) {
    if (togglingProviders.has(providerId)) return;

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
      console.error(`Failed to toggle provider ${providerId}:`, e);
      // Revert local state on error
      providers = providers.map((p) =>
        p.id === providerId ? { ...p, is_selected: !isSelected } : p
      );
    } finally {
      // Remove toggling state
      togglingProviders.delete(providerId);
      togglingProviders = new Set(togglingProviders);
    }
  }

  // Get selected providers count and names for the test validation
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

  // Toggle dropdown visibility
  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
    if (dropdownOpen) {
      activeIndex = -1;
    }
    ondropdowntoggled?.(dropdownOpen, buttonElement ? buttonElement.getBoundingClientRect() : null);
  }

  // Keyboard navigation
  function handleKeydown(event: KeyboardEvent) {
    if (!dropdownOpen) {
      if (event.key === 'ArrowDown' || event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        dropdownOpen = true;
        activeIndex = 0;
        ondropdowntoggled?.(true, buttonElement ? buttonElement.getBoundingClientRect() : null);
      }
      return;
    }

    if (event.key === 'Escape') {
      dropdownOpen = false;
      ondropdowntoggled?.(false, null);
      buttonElement?.focus();
      return;
    }

    if (event.key === 'ArrowDown') {
      event.preventDefault();
      activeIndex = (activeIndex + 1) % providers.length;
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      activeIndex = (activeIndex - 1 + providers.length) % providers.length;
    } else if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      if (activeIndex >= 0 && activeIndex < providers.length) {
        const provider = providers[activeIndex];
        handleProviderToggle(provider.id, !provider.is_selected);
      }
    }
  }
</script>

<div class="provider-selector" role="none" onkeydown={handleKeydown}>
  {#if !loading}
    {@const selectedInfo = getSelectedInfo()}

    <!-- Custom Popover Dropdown Button -->
    <button
      bind:this={buttonElement}
      class="menu-button glass"
      class:active={dropdownOpen}
      onclick={toggleDropdown}
      aria-haspopup="listbox"
      aria-expanded={dropdownOpen}
      aria-label="Select LLM providers"
      data-testid="provider-selector-button"
    >
      <span class="button-label">
        <svg class="selector-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M19 11H5M19 11C20.1046 11 21 11.8954 21 13V19C21 20.1046 20.1046 21 19 21H5C3.89543 21 3 20.1046 3 19V13C3 11.8954 3.89543 11 5 11M19 11V7C19 4.79086 17.2091 3 15 3H9C6.79086 3 5 4.79086 5 7V11M12 15V17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span class="label-text">{selectedInfo.names}</span>
        {#if selectedInfo.count > 0}
          <span class="count-badge">{selectedInfo.count}</span>
        {/if}
      </span>
      <svg class="chevron" class:rotated={dropdownOpen} viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M6 9L12 15L18 9" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>

    <!-- Custom Glassmorphic Popover Dropdown -->
    {#if dropdownOpen}
      <div 
        bind:this={dropdownElement}
        class="dropdown-panel glass-strong" 
        role="listbox" 
        aria-label="LLM Providers"
      >
        <div class="dropdown-header">
          <span>Active LLM Engines</span>
          <span class="sub">{providers.filter(p => p.is_selected).length} selected</span>
        </div>
        
        <div class="providers-list">
          {#each providers as provider, index (provider.id)}
            <div 
              class="provider-item"
              class:focused={activeIndex === index}
              class:selected={provider.is_selected}
              class:toggling={togglingProviders.has(provider.id)}
              role="option"
              aria-selected={provider.is_selected}
              tabindex="-1"
              onclick={() => handleProviderToggle(provider.id, !provider.is_selected)}
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleProviderToggle(provider.id, !provider.is_selected); } }}
            >
              <!-- Brand logo icon -->
              <div class="brand-logo" style:--logo-color={
                provider.id === 'ChatGPT' ? 'hsl(163, 75%, 45%)' :
                provider.id === 'Gemini' ? 'hsl(217, 90%, 60%)' :
                provider.id === 'Claude' ? 'hsl(14, 75%, 60%)' :
                provider.id === 'Perplexity' ? 'hsl(172, 75%, 45%)' :
                provider.id === 'DeepSeek' ? 'hsl(228, 95%, 65%)' : 'hsl(0, 0%, 80%)'
              }>
                {#if provider.id === 'ChatGPT'}
                  <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M4.5 16.5C3.7 15.3 3.2 13.7 3.5 12.2C3.8 10.7 4.8 9.4 6.2 8.8C6.1 8 6.2 7.2 6.6 6.5C7.2 5.3 8.3 4.5 9.7 4.2C11.1 3.9 12.5 4.3 13.5 5.2C14.4 4.5 15.6 4.2 16.8 4.3C18.2 4.5 19.4 5.3 20 6.5C20.6 7.2 20.8 8.1 20.7 9C21.6 9.6 22.3 10.5 22.6 11.6C23.1 13.1 22.7 14.7 21.8 15.9C21.9 16.7 21.7 17.5 21.2 18.2C20.6 19.4 19.4 20.2 18 20.5C17 20.7 15.9 20.5 15 20C14.1 20.7 12.9 21 11.7 20.8C10.3 20.6 9.1 19.8 8.5 18.6C7.9 18 7.7 17.2 7.8 16.4C6.9 15.9 6.2 15.1 5.8 14.1C5.3 12.7 5.7 11.1 6.6 9.9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                    <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="1.5"/>
                  </svg>
                {:else}
                  <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M12 3V21M3 12H21M18.364 5.63604L5.63604 18.364M18.364 18.364L5.63604 5.63604" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                {/if}
              </div>

              <!-- Details -->
              <div class="provider-details">
                <span class="name">{provider.name}</span>
                <div class="auth-row">
                  {#if provider.is_authenticated}
                    <div class="pulse-indicator" style="background: var(--success-color);"></div>
                    <span class="auth-text ready">Ready</span>
                  {:else}
                    <div class="pulse-indicator" style="background: var(--warning-color);"></div>
                    <span class="auth-text login">Login Required</span>
                  {/if}
                </div>
              </div>

              <!-- Animated Toggle Switch -->
              <div class="switch-container">
                <button 
                  type="button"
                  class="custom-switch" 
                  class:checked={provider.is_selected}
                  aria-checked={provider.is_selected}
                  role="switch"
                  aria-label={`Toggle ${provider.name}`}
                  disabled={togglingProviders.has(provider.id)}
                >
                  <span class="switch-thumb"></span>
                </button>
              </div>
            </div>
          {/each}
        </div>

        <!-- Help Info -->
        <div class="dropdown-footer">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
            <path d="M12 16V12M12 8H12.01" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <span>If "Login Required" persists, click inside the LLM panel above and sign in.</span>
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

  .menu-button {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.6rem 1.25rem 0.6rem 1.5rem;
    cursor: pointer;
    border-radius: var(--radius-full);
    font-family: var(--font-family);
    font-size: 0.95rem;
    color: #ffffff;
    min-width: 0;
    width: 100%;
    transition: all var(--transition-normal);
  }

  .menu-button:hover, .menu-button.active {
    background: hsla(220, 15%, 15%, 0.85);
    border-color: hsla(220, 95%, 60%, 0.4);
    box-shadow: var(--shadow-md), 0 0 15px hsla(220, 95%, 60%, 0.1);
    transform: translateY(-2px);
  }

  .menu-button:active {
    transform: translateY(-0.5px);
  }

  .button-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex: 1;
    min-width: 0;
  }

  .selector-icon {
    width: 18px;
    height: 18px;
    color: var(--primary-color);
    transition: color var(--transition-normal);
    flex-shrink: 0;
  }

  .menu-button:hover .selector-icon {
    color: var(--primary-hover);
  }

  .label-text {
    flex: 1;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: 500;
    letter-spacing: -0.01em;
  }

  .count-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 20px;
    height: 20px;
    padding: 0 5px;
    background: linear-gradient(135deg, var(--primary-color) 0%, hsl(220, 90%, 50%) 100%);
    color: white;
    border-radius: var(--radius-full);
    font-size: 0.7rem;
    font-weight: 700;
    box-shadow: 0 2px 6px var(--primary-glow);
    flex-shrink: 0;
  }

  .chevron {
    width: 16px;
    height: 16px;
    color: var(--text-tertiary);
    transition: transform var(--transition-spring), color var(--transition-normal);
    flex-shrink: 0;
  }

  .chevron.rotated {
    transform: rotate(180deg);
    color: var(--text-primary);
  }

  .menu-button:hover .chevron {
    color: var(--text-secondary);
  }

  /* Custom Floating Dropdown Panel */
  .dropdown-panel {
    position: absolute;
    top: calc(100% + 12px);
    left: 0;
    width: 100%;
    border-radius: var(--radius-lg);
    z-index: 1001;
    overflow: hidden;
    animation: dropdown-enter 0.25s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    transform-origin: top left;
  }

  .dropdown-header {
    padding: 1rem 1.25rem 0.75rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-family: var(--font-heading);
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary);
    border-bottom: 1px solid var(--border-color);
  }

  .dropdown-header .sub {
    font-family: var(--font-family);
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--text-tertiary);
  }

  .providers-list {
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    max-height: 280px;
    overflow-y: auto;
  }

  .provider-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.65rem 0.75rem;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-normal);
    user-select: none;
    border: 1px solid transparent;
  }

  .provider-item:hover, .provider-item.focused {
    background: hsla(220, 20%, 100%, 0.04);
    border-color: hsla(220, 20%, 100%, 0.02);
  }

  .provider-item.selected {
    background: hsla(220, 95%, 60%, 0.04);
    border-color: hsla(220, 95%, 60%, 0.08);
  }

  .provider-item.focused {
    background: hsla(220, 20%, 100%, 0.06);
    border-color: hsla(220, 20%, 100%, 0.05);
  }

  .provider-item.toggling {
    opacity: 0.6;
    pointer-events: none;
  }

  .brand-logo {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    background: hsla(220, 15%, 15%, 0.8);
    border: 1px solid var(--border-highlight);
    color: var(--logo-color, var(--text-secondary));
    box-shadow: var(--shadow-sm);
    flex-shrink: 0;
    transition: all var(--transition-normal);
  }

  .provider-item:hover .brand-logo, .provider-item.selected .brand-logo {
    border-color: var(--logo-color);
    box-shadow: 0 0 10px hsla(from var(--logo-color) h s l / 0.15);
  }

  .brand-logo svg {
    width: 18px;
    height: 18px;
  }

  .provider-details {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    min-width: 0;
  }

  .provider-details .name {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .auth-row {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }

  .auth-text {
    font-size: 0.72rem;
    font-weight: 500;
  }

  .auth-text.ready {
    color: var(--success-color);
  }

  .auth-text.login {
    color: var(--warning-color);
  }

  /* Custom Sliding Toggle Switch */
  .switch-container {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .custom-switch {
    width: 36px;
    height: 20px;
    border-radius: var(--radius-full);
    background: hsl(220, 15%, 16%);
    border: 1px solid var(--border-highlight);
    padding: 0;
    position: relative;
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.2);
    transition: all var(--transition-spring);
    pointer-events: none; /* Let the parent item handle click */
  }

  .custom-switch.checked {
    background: linear-gradient(135deg, var(--primary-color) 0%, hsl(220, 90%, 50%) 100%);
    border-color: transparent;
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1), 0 0 8px var(--primary-glow);
  }

  .switch-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: white;
    box-shadow: var(--shadow-sm);
    transition: transform var(--transition-spring);
  }

  .custom-switch.checked .switch-thumb {
    transform: translateX(16px);
  }

  .dropdown-footer {
    padding: 0.75rem 1rem;
    display: flex;
    gap: 0.5rem;
    background: hsla(220, 20%, 3%, 0.4);
    border-top: 1px solid var(--border-color);
    font-size: 0.68rem;
    color: var(--text-tertiary);
    line-height: 1.35;
  }

  .dropdown-footer svg {
    width: 12px;
    height: 12px;
    flex-shrink: 0;
    margin-top: 1px;
    color: var(--warning-color);
  }
</style>
