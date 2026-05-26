<script lang="ts">
  import { fade } from 'svelte/transition';
  import type { PanelDimension, ProviderId } from '../types';
  import { refreshProviderWebview } from '../services/providerWebviews';

  // Props
  interface Props {
    dimension: PanelDimension;
    providerName: string;
  }

  let { dimension, providerName }: Props = $props();
  let refreshing = $state(false);

  // Convert percentages for styling bounds
  function toPercent(value: number): string {
    return `${(value * 100).toFixed(2)}%`;
  }

  // Get specific themed color palette for each provider
  function getProviderColor(providerId: ProviderId): string {
    switch (providerId) {
      case 'ChatGPT':
        return '163, 75%, 45%'; // Emerald/Teal
      case 'Gemini':
        return '217, 90%, 60%';  // Google Blue
      case 'Claude':
        return '14, 75%, 60%';   // Anthropic Coral
      case 'Perplexity':
        return '172, 75%, 45%'; // Teal
      case 'DeepSeek':
        return '228, 95%, 65%';  // DeepSeek Cobalt
      case 'Ollama':
        return '0, 0%, 90%';    // Silver/White
      default:
        return '220, 10%, 60%';  // Neutral Muted
    }
  }

  // Handle panel refresh action with micro-spin state trigger
  async function handleRefresh() {
    refreshing = true;
    try {
      await refreshProviderWebview(dimension.provider_id);
    } catch (error) {
      console.error(`Failed to refresh webview for ${dimension.provider_id}:`, error);
    } finally {
      // Keep spinning for 800ms for visual satisfaction
      setTimeout(() => {
        refreshing = false;
      }, 800);
    }
  }
</script>

<div
  class="provider-panel glass"
  style:left="calc({toPercent(dimension.x)} + 8px)"
  style:top="calc({toPercent(dimension.y)} + 8px)"
  style:width="calc({toPercent(dimension.width)} - 16px)"
  style:height="calc({toPercent(dimension.height)} - 16px)"
  style:--theme-hsl={getProviderColor(dimension.provider_id)}
  data-provider-id={dimension.provider_id}
  in:fade={{ duration: 300, delay: 100 }}
  out:fade={{ duration: 200 }}
>
  <!-- Ambient border lighting effect -->
  <div class="panel-border-glow"></div>

  <!-- Header section -->
  <div class="panel-header">
    <div class="header-identity">
      <div class="status-dot"></div>
      <h3>{providerName}</h3>
    </div>
    
    <button 
      class="refresh-button" 
      class:spinning={refreshing}
      onclick={handleRefresh} 
      title={`Refresh ${providerName} webview`}
      aria-label={`Refresh ${providerName} webview`}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.3"/>
      </svg>
    </button>
  </div>

  <!-- Content & Webview container -->
  <div class="panel-content" data-webview-target>
    <!-- Interactive debug border layer -->
    <div class="debug-border"></div>
    
    <!-- Sophisticated background loading canvas -->
    <div class="webview-placeholder" aria-hidden="true">
      <div class="canvas-logo">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
      <h4>Renders direct session output</h4>
      <p>
        The {providerName} browser sandbox will overlay here. If it does not appear, ensure your authentication session is active.
      </p>
      <div class="meta-tag">
        <span>X: {toPercent(dimension.x)}</span>
        <span class="dot"></span>
        <span>Y: {toPercent(dimension.y)}</span>
      </div>
    </div>
  </div>
</div>

<style>
  /* Premium Panel Frame */
  .provider-panel {
    position: absolute;
    border-radius: var(--radius-lg) var(--radius-lg) 0 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    transition: border-color var(--transition-normal);
    will-change: left, top, width, height;
    z-index: 10;
    border: 1px solid hsla(from hsl(var(--theme-hsl)) h s l / 0.1);
  }

  .provider-panel:hover {
    border-color: hsla(from hsl(var(--theme-hsl)) h s l / 0.35);
  }

  /* Ambient border glowing panel backdrop */
  .panel-border-glow {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    border-radius: inherit;
    box-shadow: inset 0 0 20px hsla(from hsl(var(--theme-hsl)) h s l / 0.03),
                0 4px 30px rgba(0, 0, 0, 0.4);
    z-index: 0;
    transition: all var(--transition-normal);
  }

  .provider-panel:hover .panel-border-glow {
    box-shadow: inset 0 0 30px hsla(from hsl(var(--theme-hsl)) h s l / 0.06),
                0 8px 40px rgba(0, 0, 0, 0.5),
                0 0 15px hsla(from hsl(var(--theme-hsl)) h s l / 0.05);
  }

  /* Panel Header styling */
  .panel-header {
    padding: 0.85rem 1.25rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid hsla(220, 20%, 100%, 0.03);
    background: hsla(220, 18%, 6%, 0.4);
    backdrop-filter: blur(8px);
    z-index: 1;
  }

  .header-identity {
    display: flex;
    align-items: center;
    gap: 0.65rem;
  }

  .panel-header h3 {
    margin: 0;
    font-size: 0.85rem;
    font-weight: 600;
    letter-spacing: 0.03em;
    color: var(--text-primary);
    text-transform: uppercase;
  }

  /* Pulsing Theme Color Status Dot */
  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: hsl(var(--theme-hsl));
    box-shadow: 0 0 8px hsl(var(--theme-hsl));
    position: relative;
  }

  .status-dot::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: inherit;
    animation: pulse-ring 2.5s cubic-bezier(0.24, 0, 0.38, 1) infinite;
  }

  /* Action refresh button */
  .refresh-button {
    background: transparent;
    border: none;
    color: var(--text-tertiary);
    cursor: pointer;
    padding: 6px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all var(--transition-normal);
    box-shadow: none;
  }

  .refresh-button:hover {
    color: var(--text-primary);
    background: hsla(220, 20%, 100%, 0.05);
    border-color: hsla(220, 20%, 100%, 0.1);
  }

  .refresh-button svg {
    width: 14px;
    height: 14px;
    transition: transform var(--transition-normal);
  }

  .refresh-button.spinning svg {
    animation: spin-pulsing 0.8s cubic-bezier(0.4, 0, 0.2, 1) infinite;
  }

  /* Panel webview target overlay canvas */
  .panel-content {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    background: hsla(220, 20%, 3%, 0.4);
    position: relative;
    z-index: 1;
  }

  /* Webview placeholder instructions */
  .webview-placeholder {
    text-align: center;
    color: var(--text-secondary);
    padding: 2rem;
    pointer-events: none;
    max-width: 320px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.65rem;
    animation: float-gentle 4s ease-in-out infinite;
  }

  .canvas-logo {
    width: 44px;
    height: 44px;
    border-radius: 10px;
    background: hsla(from hsl(var(--theme-hsl)) h s l / 0.04);
    border: 1px dashed hsla(from hsl(var(--theme-hsl)) h s l / 0.15);
    color: hsla(from hsl(var(--theme-hsl)) h s l / 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 0.5rem;
    box-shadow: var(--shadow-sm);
  }

  .canvas-logo svg {
    width: 22px;
    height: 22px;
  }

  .webview-placeholder h4 {
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .webview-placeholder p {
    font-size: 0.82rem;
    line-height: 1.5;
    color: var(--text-secondary);
    margin: 0;
  }

  .meta-tag {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0.65rem;
    border-radius: 4px;
    background: hsla(220, 20%, 100%, 0.02);
    border: 1px solid hsla(220, 20%, 100%, 0.04);
    font-size: 0.7rem;
    color: var(--text-tertiary);
    font-family: var(--font-mono);
    margin-top: 0.5rem;
  }

  .meta-tag .dot {
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: hsla(220, 20%, 100%, 0.2);
  }

  /* Interactive border for debugging */
  .debug-border {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: 1.5px dashed hsla(from hsl(var(--theme-hsl)) h s l / 0.2);
    pointer-events: none;
    z-index: 10000;
    opacity: 0; /* Hidden by default, toggled on request */
  }
</style>
