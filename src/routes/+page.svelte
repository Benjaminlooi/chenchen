<script lang="ts">
  import { onMount, tick } from 'svelte';
  import ProviderSelector from '../components/ProviderSelector.svelte';
  import ProviderPanel from '../components/ProviderPanel.svelte';
  import PromptInput from '../components/PromptInput.svelte';
  import { tauri } from '../services/tauri';
  import { syncProviderWebviews, type PanelBounds } from '../services/providerWebviews';
  import { initPromptExecutionListener, cleanupPromptExecutionListener } from '../services/promptExecution';
  import type { LayoutConfiguration, Provider, Submission } from '../types';
  import '../app.css'; // Import global styles

  // TEMPORARY: Disable webviews for design work
  const ENABLE_WEBVIEWS = true;

  // State
  let layout = $state<LayoutConfiguration | null>(null);
  let providers = $state<Provider[]>([]);
  let layoutContainerElement = $state<HTMLElement | null>(null);
  let layoutError = $state<string | null>(null);

  // Debounce timer for layout recalculation (T152: Performance optimization)
  let layoutDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let resizeDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let resizeObserver: ResizeObserver | null = null;
  const observedResizeTargets = new Set<Element>();
  let pendingBoundsSyncFrame: number | null = null;

  // Load providers and layout on mount
  onMount(() => {
    loadProvidersAndLayout();

    // Initialize prompt execution listener
    initPromptExecutionListener();

    // Set up custom event listener for provider changes
    window.addEventListener('providers-changed', handleProvidersChanged as EventListener);

    // Update webview positions on window resize
    window.addEventListener('resize', handleResize);

    return () => {
      cleanupPromptExecutionListener();
      window.removeEventListener('providers-changed', handleProvidersChanged as EventListener);
      window.removeEventListener('resize', handleResize);
      cleanupPanelResizeObserver();
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

  // Update webview positions when window resizes
  function handleResize() {
    if (resizeDebounceTimer) {
      clearTimeout(resizeDebounceTimer);
    }

    resizeDebounceTimer = setTimeout(() => {
      scheduleWebviewPositionSync();
      resizeDebounceTimer = null;
    }, 150); // 150ms debounce delay
  }

  function scheduleWebviewPositionSync() {
    if (!ENABLE_WEBVIEWS || typeof window === 'undefined') return;

    if (pendingBoundsSyncFrame !== null) {
      window.cancelAnimationFrame(pendingBoundsSyncFrame);
    }

    pendingBoundsSyncFrame = window.requestAnimationFrame(() => {
      pendingBoundsSyncFrame = null;
      updateWebviewPositions();
    });
  }

  function initPanelResizeObserver() {
    if (!ENABLE_WEBVIEWS || !layoutContainerElement) {
      return;
    }

    if (typeof ResizeObserver === 'undefined') {
      console.warn('ResizeObserver is not supported in this environment.');
      scheduleWebviewPositionSync();
      return;
    }

    if (!resizeObserver) {
      resizeObserver = new ResizeObserver(() => {
        scheduleWebviewPositionSync();
      });
    }

    const targets = layoutContainerElement.querySelectorAll('[data-webview-target]');
    const nextObservedTargets = new Set<Element>();

    targets.forEach((target) => {
      nextObservedTargets.add(target);
      if (!observedResizeTargets.has(target)) {
        resizeObserver?.observe(target);
      }
    });

    observedResizeTargets.forEach((target) => {
      if (!nextObservedTargets.has(target)) {
        resizeObserver?.unobserve(target);
      }
    });

    observedResizeTargets.clear();
    nextObservedTargets.forEach((target) => observedResizeTargets.add(target));
  }

  function cleanupPanelResizeObserver() {
    if (resizeObserver) {
      observedResizeTargets.forEach((target) => resizeObserver?.unobserve(target));
      observedResizeTargets.clear();
      resizeObserver.disconnect();
      resizeObserver = null;
    }

    if (pendingBoundsSyncFrame !== null && typeof window !== 'undefined') {
      window.cancelAnimationFrame(pendingBoundsSyncFrame);
      pendingBoundsSyncFrame = null;
    }
  }

  function updateWebviewPositions() {
    if (!ENABLE_WEBVIEWS) return; // Skip when disabled

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
    // TODO: Handle submission status updates when event system is implemented
    console.log('Prompt submitted:', event.detail.submissions);
  }

  // Convert percentage-based PanelDimensions to pixel-based PanelBounds
  // Uses data-webview-target elements to position webviews within panel content areas
  function calculatePanelBounds(): PanelBounds[] {
    if (!layout || !layoutContainerElement) {
      return [];
    }

    const container = layoutContainerElement; // Store for TypeScript null check

    return layout.panel_dimensions
      .map((dimension) => {
        // Query for the webview target element within this specific provider panel
        const targetElement = container.querySelector(
          `[data-provider-id="${dimension.provider_id}"] [data-webview-target]`
        );

        if (!targetElement) {
          console.warn(`No webview target found for provider: ${dimension.provider_id}`);
          return null;
        }

        // Get the actual rendered bounds of the content area
        const contentRect = targetElement.getBoundingClientRect();
        console.log(`Panel bounds for ${dimension.provider_id}:`, {
          x: contentRect.x,
          y: contentRect.y,
          width: contentRect.width,
          height: contentRect.height,
        });

        return {
          providerId: dimension.provider_id,
          x: contentRect.x,
          y: contentRect.y,
          width: contentRect.width,
          height: contentRect.height,
        };
      })
      .filter((bounds): bounds is PanelBounds => bounds !== null);
  }

  // Sync webviews when layout changes (providers selected/deselected)
  $effect(() => {
    if (!ENABLE_WEBVIEWS) return; // Skip when disabled

    if (layout && layout.panel_dimensions.length > 0 && layoutContainerElement) {
      // Wait for DOM to update with new ProviderPanel components before observing
      tick().then(() => {
        initPanelResizeObserver();
        scheduleWebviewPositionSync();
      });
    } else {
      cleanupPanelResizeObserver();
    }
  });
</script>

<main class="container">
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
      Select LLMs below to view responses side-by-side
    </div>
  {/if}

  <div class="bottom-bar">
    <ProviderSelector />
    <div class="divider"></div>
    <PromptInput on:submitted={handlePromptSubmitted} />
  </div>
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

  .bottom-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 1rem;
    background: #f8f8f8;
    border-top: 1px solid #ddd;
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

    .bottom-bar {
      background: #1a1a1a;
      border-top-color: #444;
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
