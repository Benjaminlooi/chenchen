<script lang="ts">
  import { onMount, tick } from 'svelte';
  import ProviderSelector from '../components/ProviderSelector.svelte';
  import ProviderPanel from '../components/ProviderPanel.svelte';
  import PromptInput from '../components/PromptInput.svelte';
  import { tauri } from '../services/tauri';
  import { syncProviderWebviews, type PanelBounds } from '../services/providerWebviews';
  import type { LayoutConfiguration, Provider, Submission } from '../types';
  import '../app.css'; // Import global styles

  // TEMPORARY: Disable webviews for design work
  const ENABLE_WEBVIEWS = true;

  // State
  let layout = $state<LayoutConfiguration | null>(null);
  let providers = $state<Provider[]>([]);
  let layoutContainerElement = $state<HTMLElement | null>(null);
  let bottomBarElement = $state<HTMLElement | null>(null);
  let layoutError = $state<string | null>(null);

  // Debounce timers for performance optimization (T152)
  let resizeDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let resizeObserver: ResizeObserver | null = null;
  const observedResizeTargets = new Set<Element>();
  let pendingBoundsSyncFrame: number | null = null;
  let resizeObserverDebounceTimer: ReturnType<typeof setTimeout> | null = null;
  let lastSyncedBounds: string | null = null; // JSON stringified bounds for comparison
  let isSyncing = false; // Prevent re-entrant syncing

  // Load providers and layout on mount
  onMount(() => {
    loadProvidersAndLayout();

    // Set up custom event listener for provider changes
    window.addEventListener('providers-changed', handleProvidersChanged as EventListener);

    // Update webview positions on window resize
    window.addEventListener('resize', handleResize);

    return () => {
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
    updateLayout();
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
    }, 100); // 100ms debounce delay - reduced for smoother resizing
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
        // Debounce ResizeObserver to avoid firing during CSS transitions
        if (resizeObserverDebounceTimer) {
          clearTimeout(resizeObserverDebounceTimer);
        }
        resizeObserverDebounceTimer = setTimeout(() => {
          scheduleWebviewPositionSync();
          resizeObserverDebounceTimer = null;
        }, 150); // 150ms debounce - wait for CSS transitions to complete
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

    if (resizeObserverDebounceTimer) {
      clearTimeout(resizeObserverDebounceTimer);
      resizeObserverDebounceTimer = null;
    }

    if (pendingBoundsSyncFrame !== null && typeof window !== 'undefined') {
      window.cancelAnimationFrame(pendingBoundsSyncFrame);
      pendingBoundsSyncFrame = null;
    }
  }

  function updateWebviewPositions() {
    if (!ENABLE_WEBVIEWS) return; // Skip when disabled
    if (isSyncing) return; // Prevent re-entrant syncing

    if (layout && layout.panel_dimensions.length > 0 && layoutContainerElement) {
      const bounds = calculatePanelBounds();

      // Don't sync if we get no valid bounds or incomplete bounds
      // This prevents disposing all webviews due to CSS transition intermediate values
      if (bounds.length === 0 || bounds.length < layout.panel_dimensions.length) {
        console.log(`Skipping webview sync - incomplete bounds (${bounds.length}/${layout.panel_dimensions.length})`);
        return;
      }

      // Check if bounds have actually changed
      const boundsKey = JSON.stringify(
        bounds.map((b) => ({
          id: b.providerId,
          x: Math.round(b.x),
          y: Math.round(b.y),
          w: Math.round(b.width),
          h: Math.round(b.height),
        }))
      );

      if (boundsKey === lastSyncedBounds) {
        return; // No meaningful change, skip sync
      }

      isSyncing = true;
      lastSyncedBounds = boundsKey;

      syncProviderWebviews(bounds)
        .catch((error) => {
          console.error('Failed to update webview positions:', error);
        })
        .finally(() => {
          isSyncing = false;
        });
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
    const MIN_WEBVIEW_HEIGHT = 50; // Minimum reasonable height for a webview
    const MIN_WEBVIEW_WIDTH = 50; // Minimum reasonable width for a webview

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

        // Filter out invalid bounds (too small - likely from CSS transition intermediate frames)
        if (contentRect.width < MIN_WEBVIEW_WIDTH || contentRect.height < MIN_WEBVIEW_HEIGHT) {
          console.log(`Skipping ${dimension.provider_id} - bounds too small during transition:`, {
            width: contentRect.width,
            height: contentRect.height,
          });
          return null;
        }

        console.log(`Panel bounds for ${dimension.provider_id}:`, {
          x: contentRect.x,
          y: contentRect.y,
          width: contentRect.width,
          height: contentRect.height,
          windowInnerWidth: window.innerWidth,
          windowInnerHeight: window.innerHeight,
        });

        return {
          providerId: dimension.provider_id,
          // x: Math.floor(contentRect.left),
          // y: Math.floor(contentRect.top),
          // width: (Math.ceil(contentRect.right) - Math.floor(contentRect.left)),
          // height: (Math.ceil(contentRect.bottom) - Math.floor(contentRect.top)),
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

  <div class="bottom-bar" bind:this={bottomBarElement}>
    <ProviderSelector />
    <div class="divider"></div>
    <PromptInput onsubmitted={handlePromptSubmitted} />
  </div>
</main>

<style>
  :global(body) {
    font-family: 'Inter', sans-serif;
    font-size: 16px;
    line-height: 1.5;
    font-weight: 400;
    color: #ffffff;
    background-color: #0a0a0a;
    margin: 0;
    padding: 0;
    overflow: hidden; /* Prevent scrollbars on body */
  }

  .container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    background: radial-gradient(circle at 15% 50%, rgba(59, 130, 246, 0.08), transparent 25%),
                radial-gradient(circle at 85% 30%, rgba(139, 92, 246, 0.08), transparent 25%);
    background-size: 120% 120%;
    animation: background-drift 20s ease-in-out infinite alternate;
    position: relative;
  }

  @keyframes background-drift {
    0% {
      background-position: 0% 50%;
    }
    100% {
      background-position: 100% 50%;
    }
  }

  /* Mesh gradient background effect */
  .container::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23ffffff' fill-opacity='0.03'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
    pointer-events: none;
    z-index: 0;
  }

  .bottom-bar {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 1.5rem 2rem 2rem;
    background: linear-gradient(to top, rgba(10, 10, 10, 0.95) 0%, rgba(10, 10, 10, 0.8) 60%, transparent 100%);
    flex-shrink: 0;
    position: relative;
    z-index: 1000;
  }

  .divider {
    width: 1px;
    height: 24px;
    background: rgba(255, 255, 255, 0.1);
  }

  .layout-container {
    position: relative;
    flex: 1;
    background: transparent; /* Let container gradient show through */
    overflow: hidden;
    z-index: 1;
  }

  .layout-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: rgba(255, 255, 255, 0.3);
    font-size: 1.1rem;
    font-weight: 300;
    letter-spacing: 0.02em;
  }
</style>
