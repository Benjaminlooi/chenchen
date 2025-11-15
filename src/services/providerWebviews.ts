import { Webview, type WebviewOptions } from '@tauri-apps/api/webview';
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/dpi';
import { getCurrentWindow, type Window } from '@tauri-apps/api/window';
import type { ProviderId, WebviewInfo } from '../types';

export interface PanelBounds {
  providerId: ProviderId;
  x: number;
  y: number;
  width: number;
  height: number;
}

interface ActiveWebview {
  webview: Webview;
  info: WebviewInfo;
}

const activeWebviews = new Map<ProviderId, ActiveWebview>();
const pendingCreations = new Map<ProviderId, Promise<void>>();
let windowHandle: Window | null = null;

function getAppWindow(): Window {
  if (!windowHandle) {
    windowHandle = getCurrentWindow();
  }
  return windowHandle;
}

function toLogicalBounds(bounds: PanelBounds) {
  return {
    x: Math.round(bounds.x),
    y: Math.round(bounds.y),
    width: Math.round(bounds.width),
    height: Math.round(bounds.height),
  };
}

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

function generateDataStoreId(): string {
  // Generate a UUID v4 for macOS data store identifier
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0;
    const v = c === 'x' ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

function buildWebviewInfo(providerId: ProviderId): WebviewInfo {
  const label = `${providerId.toLowerCase()}-webview`;
  const url = getProviderUrl(providerId);

  return {
    provider_id: providerId,
    label,
    url,
    is_persistent: true,
    data_store_id: generateDataStoreId(), // For macOS
  };
}

export async function syncProviderWebviews(bounds: PanelBounds[]): Promise<void> {
  if (typeof window === 'undefined') {
    return;
  }

  const validProviders = new Set(bounds.map((bound) => bound.providerId));

  for (const providerId of Array.from(activeWebviews.keys())) {
    if (!validProviders.has(providerId)) {
      await destroyProviderWebview(providerId);
    }
  }

  for (const bound of bounds) {
    if (bound.width <= 0 || bound.height <= 0) {
      continue;
    }
    await ensureWebviewForBounds(bound);
  }
}

export async function focusProviderWebview(providerId: ProviderId): Promise<void> {
  const pending = pendingCreations.get(providerId);
  if (pending) {
    await pending;
  }

  const entry = activeWebviews.get(providerId);
  if (!entry) {
    return;
  }

  await entry.webview.show();
  await entry.webview.setFocus();
}

export async function disposeAllProviderWebviews(): Promise<void> {
  for (const providerId of Array.from(activeWebviews.keys())) {
    await destroyProviderWebview(providerId);
  }
}

async function ensureWebviewForBounds(bound: PanelBounds): Promise<void> {
  if (!activeWebviews.has(bound.providerId)) {
    let pending = pendingCreations.get(bound.providerId);
    if (!pending) {
      pending = createProviderWebview(bound);
      pendingCreations.set(bound.providerId, pending);
    }

    try {
      await pending;
    } finally {
      pendingCreations.delete(bound.providerId);
    }
  }

  const entry = activeWebviews.get(bound.providerId);
  if (!entry) {
    return;
  }

  // Ensure manual sizing stays in control even after window resizes
  await entry.webview.setAutoResize(false);

  const { x, y, width, height } = toLogicalBounds(bound);
  console.log(`Positioning webview ${bound.providerId} at (${x}, ${y}) size ${width}x${height}`);
  await entry.webview.setPosition(new LogicalPosition(x, y));
  await entry.webview.setSize(new LogicalSize(width, height));
  await entry.webview.show();
}

async function createProviderWebview(bound: PanelBounds): Promise<void> {
  try {
    const info = buildWebviewInfo(bound.providerId);
    const { x, y, width, height } = toLogicalBounds(bound);
    const windowHandle = getAppWindow();

    console.log(`Creating webview ${bound.providerId}:`, { x, y, width, height, url: info.url });

    const options: WebviewOptions = {
      url: info.url,
      x,
      y,
      width,
      height,
      focus: false,
      visible: false, // Start hidden, show after positioning
    };

    if (info.data_store_id) {
      const bytes = uuidToBytes(info.data_store_id);
      if (bytes.length === 16) {
        options.dataStoreIdentifier = bytes;
      }
    }

    const webview = new Webview(windowHandle, info.label, options);
    await waitForCreation(webview);

    // Disable auto-resize so manual bounds fully control layout (T153)
    await webview.setAutoResize(false);

    // Ensure position and size are set before showing
    await webview.setPosition(new LogicalPosition(x, y));
    await webview.setSize(new LogicalSize(width, height));
    await webview.show();

    activeWebviews.set(bound.providerId, { webview, info });
    console.log(`Webview ${bound.providerId} created and positioned at (${x}, ${y}) size ${width}x${height}`);
  } catch (error) {
    console.error(`Failed to create provider webview for ${bound.providerId}`, error);
  }
}

async function destroyProviderWebview(providerId: ProviderId): Promise<void> {
  const entry = activeWebviews.get(providerId);
  if (!entry) {
    return;
  }

  try {
    await entry.webview.hide();
    await entry.webview.close();
  } catch (error) {
    console.warn(`Failed to close provider webview ${providerId}`, error);
  } finally {
    activeWebviews.delete(providerId);
  }
}

async function waitForCreation(webview: Webview): Promise<void> {
  await new Promise<void>(async (resolve, reject) => {
    const unlistenCreated = await webview.once('tauri://created', () => {
      cleanup();
      resolve();
    });
    const unlistenError = await webview.once('tauri://error', (event) => {
      cleanup();
      reject(event.payload ?? 'webview creation error');
    });

    function cleanup() {
      unlistenCreated();
      unlistenError();
    }
  }).catch((error) => {
    console.error('Webview initialization error', error);
  });
}

function uuidToBytes(uuid: string): number[] {
  const sanitized = uuid.replace(/-/g, '');
  if (sanitized.length !== 32) {
    return [];
  }

  const bytes: number[] = [];
  for (let i = 0; i < sanitized.length; i += 2) {
    bytes.push(parseInt(sanitized.slice(i, i + 2), 16));
  }
  return bytes;
}
