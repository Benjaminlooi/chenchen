import { afterEach } from 'vitest';
import { clearMocks } from '@tauri-apps/api/mocks';

// Clear Tauri IPC mocks after each test to prevent test pollution
afterEach(() => {
  clearMocks();
});
