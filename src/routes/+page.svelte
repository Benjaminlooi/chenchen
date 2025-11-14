<script lang="ts">
  import { onMount } from 'svelte';
  import ProviderSelector from '../components/ProviderSelector.svelte';
  import ProviderPanel from '../components/ProviderPanel.svelte';
  import PromptInput from '../components/PromptInput.svelte';
  import StatusDisplay from '../components/StatusDisplay.svelte';
  import { tauri } from '../services/tauri';
  import type { LayoutConfiguration, Provider, Submission } from '../types';
  import '../app.css'; // Import global styles

  // State
  let layout = $state<LayoutConfiguration | null>(null);
  let providers = $state<Provider[]>([]);
  let layoutError = $state<string | null>(null);
  let submissions = $state<Submission[]>([]);

  // Debounce timer for layout recalculation (T152: Performance optimization)
  let layoutDebounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Load providers and layout on mount
  onMount(() => {
    loadProvidersAndLayout();

    // Set up custom event listener for provider changes
    window.addEventListener('providers-changed', handleProvidersChanged as EventListener);

    return () => {
      window.removeEventListener('providers-changed', handleProvidersChanged as EventListener);
    };
  });

  async function loadProvidersAndLayout() {
    try {
      providers = await tauri.getProviders();
      await updateLayout();
    } catch (error) {
      console.error('Failed to load providers:', error);
    }
  }

  async function updateLayout() {
    const selectedProviders = providers.filter((p) => p.is_selected);

    if (selectedProviders.length === 0) {
      layout = null;
      return;
    }

    try {
      layout = await tauri.getLayoutConfiguration();
      layoutError = null;
    } catch (error) {
      console.error('Failed to get layout configuration:', error);
      layoutError = 'Failed to calculate layout';
      layout = null;
    }
  }

  // Debounced layout update for performance (T152)
  function debouncedUpdateLayout() {
    if (layoutDebounceTimer) {
      clearTimeout(layoutDebounceTimer);
    }

    layoutDebounceTimer = setTimeout(() => {
      updateLayout();
      layoutDebounceTimer = null;
    }, 150); // 150ms debounce delay
  }

  function handleProvidersChanged(event: CustomEvent) {
    providers = event.detail.providers;
    debouncedUpdateLayout();
  }

  function getProviderName(providerId: string): string {
    const provider = providers.find((p) => p.id === providerId);
    return provider?.name || providerId;
  }

  // T124: Handle prompt submission
  function handlePromptSubmitted(event: CustomEvent<{ submissions: Submission[] }>) {
    submissions = event.detail.submissions;
  }
</script>

<main class="container">
  <h1>ChenChen - Multi-LLM Prompt</h1>
  <p class="subtitle">Send prompts to multiple LLMs simultaneously</p>

  <ProviderSelector />

  <!-- T124: Prompt input component -->
  <PromptInput on:submitted={handlePromptSubmitted} />

  <!-- T124: Status display component -->
  {#if submissions.length > 0}
    <StatusDisplay {submissions} />
  {/if}

  <!-- Provider panels in split-screen layout -->
  {#if layout && layout.panel_dimensions.length > 0}
    <div class="layout-container">
      {#each layout.panel_dimensions as dimension (dimension.provider_id)}
        <ProviderPanel
          {dimension}
          providerName={getProviderName(dimension.provider_id)}
        />
      {/each}
    </div>
  {:else if layoutError}
    <div class="layout-error">{layoutError}</div>
  {:else}
    <div class="layout-placeholder">
      <p>Select one or more providers to begin</p>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #0f0f0f;
    background-color: #f6f6f6;
    margin: 0;
    padding: 0;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    text-align: center;
    font-size: 2rem;
    margin-bottom: 0.5rem;
    color: #333;
  }

  .subtitle {
    text-align: center;
    color: #666;
    margin-bottom: 2rem;
  }

  .layout-container {
    position: relative;
    width: 100%;
    height: 600px;
    background: #fff;
    border: 2px solid #ddd;
    border-radius: 8px;
    margin-top: 2rem;
    overflow: hidden;
  }

  .layout-placeholder,
  .layout-error {
    padding: 3rem;
    text-align: center;
    color: #999;
    font-style: italic;
    margin-top: 2rem;
  }

  .layout-error {
    color: #c33;
    background: #fee;
    border: 1px solid #fcc;
    border-radius: 4px;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    h1 {
      color: #f6f6f6;
    }

    .subtitle {
      color: #ccc;
    }

    .layout-container {
      background: #1a1a1a;
      border-color: #444;
    }

    .layout-placeholder {
      color: #888;
    }

    .layout-error {
      background: #3a1a1a;
      border-color: #6a2a2a;
      color: #ff6666;
    }
  }
</style>
