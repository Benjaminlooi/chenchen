<script lang="ts">
  import { onMount } from 'svelte';
  import ProviderSelector from '../components/ProviderSelector.svelte';
  import ProviderPanel from '../components/ProviderPanel.svelte';
  import PromptInput from '../components/PromptInput.svelte';
  import { tauri } from '../services/tauri';
  import { syncProviderWebviews, type PanelBounds } from '../services/providerWebviews';
  import type { LayoutConfiguration, Provider, Submission } from '../types';
  import '../app.css'; // Import global styles

  // State
  let layout = $state<LayoutConfiguration | null>(null);
  let providers = $state<Provider[]>([]);
  let submissions = $state<Submission[]>([]);
  let layoutContainerElement = $state<HTMLElement | null>(null);

  // Debounce timer for layout recalculation (T152: Performance optimization)
  let layoutDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let scrollDebounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Load providers and layout on mount
  onMount(async () => {
    loadProvidersAndLayout();

    // Set up custom event listener for provider changes
    window.addEventListener('providers-changed', handleProvidersChanged as EventListener);

    // Update webview positions on scroll
    window.addEventListener('scroll', handleScroll);

    return () => {
      window.removeEventListener('providers-changed', handleProvidersChanged as EventListener);
      window.removeEventListener('scroll', handleScroll);
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
      console.log('Layout configuration from backend:', layout);
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

  // Update webview positions when scrolling
  function handleScroll() {
    if (scrollDebounceTimer) {
      clearTimeout(scrollDebounceTimer);
    }

    scrollDebounceTimer = setTimeout(() => {
      updateWebviewPositions();
      scrollDebounceTimer = null;
    }, 16); // ~60fps
  }

  function updateWebviewPositions() {
    if (layout && layout.panel_dimensions.length > 0 && layoutContainerElement) {
      const bounds = calculatePanelBounds();
      if (bounds.length > 0) {
        syncProviderWebviews(bounds).catch((error) => {
          console.error('Failed to update webview positions:', error);
        });
      }
    }
  }

  function getProviderName(providerId: string): string {
    const provider = providers.find((p) => p.id === providerId);
    return provider?.name || providerId;
  }

  // T124: Handle prompt submission
  async function handlePromptSubmitted(event: CustomEvent<{ submissions: Submission[] }>) {
    submissions = event.detail.submissions;
  }

  // Convert percentage-based PanelDimensions to pixel-based PanelBounds
  function calculatePanelBounds(): PanelBounds[] {
    if (!layout || !layoutContainerElement) {
      return [];
    }

    const rect = layoutContainerElement.getBoundingClientRect();
    const containerWidth = rect.width;
    const containerHeight = rect.height;

    // rect.x and rect.y give us the offset from the window's top-left
    const containerOffsetX = rect.x;
    const containerOffsetY = rect.y;

    return layout.panel_dimensions.map((dimension) => ({
      providerId: dimension.provider_id,
      x: containerOffsetX + (dimension.x * containerWidth),
      y: containerOffsetY + (dimension.y * containerHeight),
      width: dimension.width * containerWidth,
      height: dimension.height * containerHeight,
    }));
  }

  // Sync webviews when layout changes (providers selected/deselected)
  $effect(() => {
    if (layout && layout.panel_dimensions.length > 0 && layoutContainerElement) {
      const bounds = calculatePanelBounds();
      if (bounds.length > 0) {
        syncProviderWebviews(bounds)
          .then(() => {
            console.log('Provider webviews synced successfully');
          })
          .catch((error) => {
            console.error('Failed to sync provider webviews:', error);
          });
      }
    }
  });
</script>

<main class="container">
  <div class="top-bar">
    <ProviderSelector />
    <div class="divider"></div>
    <PromptInput on:submitted={handlePromptSubmitted} />
  </div>

  <!-- Provider panels in split-screen layout -->
  {#if layout && layout.panel_dimensions.length > 0}
    <div class="layout-container" bind:this={layoutContainerElement}>
      {#each layout.panel_dimensions as dimension (dimension.provider_id)}
        <ProviderPanel
          {dimension}
          providerName={getProviderName(dimension.provider_id)}
        />
      {/each}
    </div>
  {:else}
    <div class="layout-placeholder">
      Select LLMs above to view responses side-by-side
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
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .top-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 1rem;
    background: #f8f8f8;
    border-bottom: 1px solid #ddd;
    flex-shrink: 0;
  }

  .divider {
    width: 1px;
    height: 30px;
    background: #ddd;
  }

  .layout-container {
    position: relative;
    flex: 1;
    background: #fff;
    overflow: hidden;
  }

  .layout-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #999;
    font-size: 1.1rem;
    font-style: italic;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    .top-bar {
      background: #1a1a1a;
      border-bottom-color: #444;
    }

    .divider {
      background: #444;
    }

    .layout-container {
      background: #1a1a1a;
    }

    .layout-placeholder {
      color: #888;
    }
  }
</style>
