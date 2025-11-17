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
      default:
        return '#666';
    }
  }
</script>

<div
  class="provider-panel"
  style:left={toPercent(dimension.x)}
  style:top={toPercent(dimension.y)}
  style:width={toPercent(dimension.width)}
  style:height={toPercent(dimension.height)}
  style:border-color={getProviderColor(dimension.provider_id)}
  data-provider-id={dimension.provider_id}
  in:fade={{ duration: 150, delay: 100 }}
  out:fade={{ duration: 150 }}
>
  <div class="panel-header" style:background={getProviderColor(dimension.provider_id)}>
    <h3>{providerName}</h3>
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
    border: 2px solid;
    border-radius: 8px;
    background: white;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    transition: left 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                top 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                width 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94),
                height 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
    will-change: left, top, width, height;
  }

  .panel-header {
    padding: 0.5rem 1rem;
    color: white;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .panel-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .panel-content {
    height: calc(100% - 2.5rem);
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f9f9f9;
    position: relative;
  }

  .webview-placeholder {
    text-align: center;
    color: #666;
    padding: 0 1rem;
    pointer-events: none;
  }

  .webview-placeholder p {
    margin: 0.5rem 0;
  }

  .hint {
    font-size: 0.85rem;
    color: #888;
  }

  .debug-info {
    font-size: 0.75rem;
    color: #999;
    font-family: monospace;
  }

  .debug-border {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: 3px dashed rgba(255, 0, 0, 0.5);
    pointer-events: none;
    z-index: 9999;
  }

  @media (prefers-color-scheme: dark) {
    .provider-panel {
      background: #2a2a2a;
      border-color: #444;
    }

    .panel-content {
      background: #1a1a1a;
    }

    .webview-placeholder {
      color: #ccc;
    }

    .hint {
      color: #aaa;
    }
  }
</style>
