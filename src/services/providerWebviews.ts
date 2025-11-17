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

  const activeProviderIds = new Set<ProviderId>();

  for (const bound of bounds) {
    if (bound.width <= 0 || bound.height <= 0) {
      continue;
    }

    activeProviderIds.add(bound.providerId);
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

  const providersToDispose = [...createdWebviews].filter(
    (providerId) => !activeProviderIds.has(providerId)
  );

  for (const providerId of providersToDispose) {
    try {
      await disposeProviderWebview(providerId);
      createdWebviews.delete(providerId);
      console.log(`[ProviderWebviews] Disposed webview for ${providerId}`);
    } catch (error) {
      console.error(`[ProviderWebviews] Failed to dispose webview for ${providerId}:`, error);
    }
  }
}

async function disposeProviderWebview(providerId: ProviderId): Promise<void> {
  await invoke('dispose_provider_webview', { providerId });
}
