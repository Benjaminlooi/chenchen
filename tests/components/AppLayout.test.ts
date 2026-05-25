import '@testing-library/jest-dom/vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import Page from '../../src/routes/+page.svelte';
import { tauri } from '../../src/services/tauri';
import { LayoutType, ProviderId } from '../../src/types';

vi.mock('../../src/services/providerWebviews', () => ({
  syncProviderWebviews: vi.fn().mockResolvedValue(undefined),
  refreshProviderWebview: vi.fn().mockResolvedValue(undefined),
}));

vi.mock('../../src/services/tauri', () => ({
  tauri: {
    getProviders: vi.fn(),
    getLayoutConfiguration: vi.fn(),
    getSubmissionStatus: vi.fn(),
    submitPrompt: vi.fn(),
    updateProviderSelection: vi.fn(),
  },
}));

describe('App layout', () => {
  it('reserves a fixed control console outside the webview workspace', async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue([
      {
        id: ProviderId.ChatGPT,
        name: 'ChatGPT',
        url: 'https://chat.openai.com/',
        is_selected: true,
        is_authenticated: true,
        selector_config_id: 'ChatGPT',
      },
    ]);
    vi.mocked(tauri.getLayoutConfiguration).mockResolvedValue({
      provider_count: 1,
      layout_type: LayoutType.Full,
      panel_dimensions: [
        {
          provider_id: ProviderId.ChatGPT,
          x: 0,
          y: 0,
          width: 1,
          height: 1,
        },
      ],
    });

    const { container } = render(Page);

    await screen.findByTestId('provider-selector-button');
    await waitFor(() => {
      expect(container.querySelector('.layout-container')).toBeInTheDocument();
    });

    expect(container.querySelector('.app-shell')).toBeInTheDocument();
    expect(container.querySelector('.webview-workspace')).toBeInTheDocument();
    expect(container.querySelector('.control-console')).toBeInTheDocument();
    expect(container.querySelector('.bottom-bar')).not.toBeInTheDocument();
  });
});
