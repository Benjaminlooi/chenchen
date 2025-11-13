<script lang="ts">
  import type { PanelDimension, ProviderId } from '../types';

  // Props
  interface Props {
    dimension: PanelDimension;
    providerName: string;
  }

  let { dimension, providerName }: Props = $props();

  /**
   * Converts percentage dimensions (0.0-1.0) to CSS percentages
   */
  function toPercent(value: number): string {
    return `${(value * 100).toFixed(2)}%`;
  }

  /**
   * Get background color for each provider
   */
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

<!-- Provider panel with absolute positioning based on layout dimensions -->
<div
  class="provider-panel"
  style:left={toPercent(dimension.x)}
  style:top={toPercent(dimension.y)}
  style:width={toPercent(dimension.width)}
  style:height={toPercent(dimension.height)}
  style:border-color={getProviderColor(dimension.provider_id)}
  data-provider-id={dimension.provider_id}
>
  <div class="panel-header" style:background={getProviderColor(dimension.provider_id)}>
    <h3>{providerName}</h3>
  </div>

  <div class="panel-content">
    <!-- Placeholder for provider webview/iframe -->
    <div class="webview-placeholder">
      <p>Provider webview will be loaded here</p>
      <p class="hint">URL: {dimension.provider_id}</p>
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
    transition: all 0.3s ease;
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
  }

  .webview-placeholder {
    text-align: center;
    color: #666;
  }

  .webview-placeholder p {
    margin: 0.5rem 0;
  }

  .hint {
    font-size: 0.875rem;
    font-style: italic;
    color: #999;
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
      color: #888;
    }
  }
</style>
