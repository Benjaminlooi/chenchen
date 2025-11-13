import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type { CommandError } from '../types';

/**
 * Type-safe wrapper for Tauri invoke() with error handling
 *
 * Usage:
 * ```ts
 * import { invoke } from './services/tauri';
 *
 * const providers = await invoke<Provider[]>('get_providers');
 * ```
 */
export async function invoke<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    const result = await tauriInvoke<T>(command, args);
    return result;
  } catch (error) {
    // Re-throw as CommandError if it matches the structure
    if (
      error &&
      typeof error === 'object' &&
      'code' in error &&
      'message' in error
    ) {
      throw error as CommandError;
    }

    // Otherwise wrap in a generic CommandError
    throw {
      code: 'UnknownError',
      message: error instanceof Error ? error.message : String(error)
    } as CommandError;
  }
}

/**
 * Type-safe wrapper for Tauri event listeners
 * Returns an unlisten function to clean up the listener
 */
export async function listen<T>(
  event: string,
  handler: (payload: T) => void
): Promise<() => void> {
  const { listen: tauriListen } = await import('@tauri-apps/api/event');
  const unlisten = await tauriListen<T>(event, (event) => {
    handler(event.payload);
  });
  return unlisten;
}

/**
 * Emit a custom event
 */
export async function emit<T>(event: string, payload: T): Promise<void> {
  const { emit: tauriEmit } = await import('@tauri-apps/api/event');
  await tauriEmit(event, payload);
}
