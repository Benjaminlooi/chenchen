<script lang="ts">
  import { fade } from 'svelte/transition';
  import type { PanelDimension, ProviderId } from '../types';

  interface Props {
    dimension: PanelDimension;
    providerName: string;
  }

  let { dimension, providerName }: Props = $props();

  function toPercent(value: number): string {
    return `${(value * 100).toFixed(2)}%`;
  }

  function getProviderColor(providerId: ProviderId): string {
    switch (providerId) {
      case 'ChatGPT':
        return '#10a37f';
      case 'Gemini':
        return '#4285f4';
      case 'Claude':
        return '#d97757';
      case 'Perplexity':
        return '#22b39f';
      case 'DeepSeek':
        return '#4d6bfe';
      case 'Ollama':
        return '#ffffff';
      default:
        return '#666';
    }
  }

  import { refreshProviderWebview } from '../services/providerWebviews';

  async function handleRefresh() {
    await refreshProviderWebview(dimension.provider_id);
  }
</script>

<div
  class="provider-panel"
  style:left="calc({toPercent(dimension.x)} + 8px)"
  style:top="calc({toPercent(dimension.y)} + 8px)"
  style:width="calc({toPercent(dimension.width)} - 16px)"
  style:height="calc({toPercent(dimension.height)} - 16px)"
  style:--provider-color={getProviderColor(dimension.provider_id)}
  data-provider-id={dimension.provider_id}
  in:fade={{ duration: 300, delay: 100 }}
  out:fade={{ duration: 200 }}
>
  <div class="panel-header">
    <h3>{providerName}</h3>
    <button class="refresh-button" onclick={handleRefresh} title="Refresh Webview" aria-label="Refresh Webview">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.3"/>
      </svg>
    </button>
  </div>

  <div class="panel-content" data-webview-target>
    <!-- Debug border to visualize panel bounds -->
    <div class="debug-border"></div>
    <div class="webview-placeholder" aria-hidden="true">
      <p>
        The {providerName} site renders directly inside this area. If it fails to
        appear, make sure you're logged in.
      </p>
      <p class="hint">This panel is fully interactive once the webview loads.</p>
      <p class="debug-info">
        Position: {toPercent(dimension.x)}, {toPercent(dimension.y)}<br/>
        Size: {toPercent(dimension.width)} × {toPercent(dimension.height)}
      </p>
    </div>
  </div>
</div>

<style>
  .provider-panel {
    position: absolute;
    border-radius: 16px;
    background: rgba(30, 30, 30, 0.6);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    overflow: hidden;
    box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
    transition: left 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                top 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                width 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                height 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    will-change: left, top, width, height;
    display: flex;
    flex-direction: column;
  }

  .panel-header {
    padding: 0.75rem 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(255, 255, 255, 0.02);
  }

  .panel-header h3 {
    margin: 0;
    font-size: 0.85rem;
    font-weight: 600;
    letter-spacing: 0.02em;
    color: rgba(255, 255, 255, 0.9);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.35rem 0.75rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 9999px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(4px);
  }

  .refresh-button {
    background: transparent;
    border: none;
    color: rgba(255, 255, 255, 0.4);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .refresh-button:hover {
    color: rgba(255, 255, 255, 0.9);
    background: rgba(255, 255, 255, 0.1);
  }

  .refresh-button svg {
    width: 14px;
    height: 14px;
  }

  /* Provider indicator dot */
  .panel-header h3::before {
    content: '';
    display: block;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--provider-color, #666);
    box-shadow: 0 0 8px var(--provider-color, #666);
  }

  .panel-content {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.2);
    position: relative;
  }

  .webview-placeholder {
    text-align: center;
    color: rgba(255, 255, 255, 0.5);
    padding: 0 1.5rem;
    pointer-events: none;
    max-width: 300px;
  }

  .webview-placeholder p {
    margin: 0.5rem 0;
    line-height: 1.5;
  }

  .hint {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.3);
  }

  .debug-info {
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.2);
    font-family: 'JetBrains Mono', monospace;
    margin-top: 1rem;
  }

  .debug-border {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: 2px dashed rgba(255, 255, 255, 0.1);
    pointer-events: none;
    z-index: 9999;
    opacity: 0; /* Hidden by default, useful for debugging */
  }
</style>
