import { invoke } from '@tauri-apps/api/core';
import type { ProviderId } from '../types';

export interface PanelBounds {
  providerId: ProviderId;
  x: number;
  y: number;
  width: number;
  height: number;
}

// Track which webviews have been created
const createdWebviews = new Set<ProviderId>();

function getProviderUrl(providerId: ProviderId): string {
  switch (providerId) {
    case 'ChatGPT':
      return 'https://chat.openai.com';
    case 'Gemini':
      return 'https://gemini.google.com';
    case 'Claude':
      return 'https://claude.ai';
    default:
      throw new Error(`Unknown provider ID: ${providerId}`);
  }
}

export async function syncProviderWebviews(bounds: PanelBounds[]): Promise<void> {
  if (typeof window === 'undefined') {
    return;
  }

  console.log('[ProviderWebviews] Syncing webviews via Rust backend', bounds);

  for (const bound of bounds) {
    if (bound.width <= 0 || bound.height <= 0) {
      continue;
    }

    const url = getProviderUrl(bound.providerId);

    try {
      await invoke('sync_provider_webview', {
        providerId: bound.providerId,
        url,
        x: bound.x,
        y: bound.y,
        width: bound.width,
        height: bound.height,
      });

      createdWebviews.add(bound.providerId);
      console.log(`[ProviderWebviews] Synced webview for ${bound.providerId}`);
    } catch (error) {
      console.error(`[ProviderWebviews] Failed to sync webview for ${bound.providerId}:`, error);
    }
  }
}
