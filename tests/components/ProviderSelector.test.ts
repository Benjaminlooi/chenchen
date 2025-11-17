import '@testing-library/jest-dom/vitest';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import ProviderSelector from '../../src/components/ProviderSelector.svelte';
import { tauri } from '../../src/services/tauri';
import { ProviderId } from '../../src/types';

// Mock the tauri service
vi.mock('../../src/services/tauri', () => ({
  tauri: {
    getProviders: vi.fn(),
    updateProviderSelection: vi.fn(),
  },
}));

vi.mock('../../src/services/providerWebviews', () => ({
  focusProviderWebview: vi.fn(),
}));

describe('ProviderSelector', () => {
  const mockProviders = [
    {
      id: ProviderId.ChatGPT,
      name: 'ChatGPT',
      url: 'https://chat.openai.com/',
      is_selected: false,
      is_authenticated: true,
      selector_config_id: 'ChatGPT',
    },
    {
      id: ProviderId.Gemini,
      name: 'Gemini',
      url: 'https://gemini.google.com/',
      is_selected: false,
      is_authenticated: true,
      selector_config_id: 'Gemini',
    },
    {
      id: ProviderId.Claude,
      name: 'Claude',
      url: 'https://claude.ai/',
      is_selected: false,
      is_authenticated: false,
      selector_config_id: 'Claude',
    },
  ];

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('loads and displays providers on mount', async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue(mockProviders);

    render(ProviderSelector);

    expect(await screen.findByText('ChatGPT')).toBeInTheDocument();
    expect(screen.getByText('Gemini')).toBeInTheDocument();
    expect(screen.getByText('Claude')).toBeInTheDocument();
  });

  it('displays loading state initially', () => {
    vi.mocked(tauri.getProviders).mockImplementation(
      () => new Promise(() => {}) // Never resolves
    );

    render(ProviderSelector);

    expect(screen.getByText('Loading providers...')).toBeInTheDocument();
  });

  it('displays error when loading fails', async () => {
    vi.mocked(tauri.getProviders).mockRejectedValue(
      new Error('Failed to load')
    );

    render(ProviderSelector);

    expect(
      await screen.findByText(/Failed to load/i)
    ).toBeInTheDocument();
  });

  it('calls updateProviderSelection when checkbox is toggled', async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue(mockProviders);
    vi.mocked(tauri.updateProviderSelection).mockResolvedValue({
      ...mockProviders[0],
      is_selected: true,
    });

    render(ProviderSelector);

    const checkbox = (await screen.findByTestId(
      'provider-checkbox-ChatGPT'
    )) as HTMLInputElement;

    await fireEvent.click(checkbox);

    expect(tauri.updateProviderSelection).toHaveBeenCalledWith(
      ProviderId.ChatGPT,
      true
    );
  });

  it('displays error when update fails and reverts checkbox', async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue(mockProviders);
    vi.mocked(tauri.updateProviderSelection).mockRejectedValue(
      new Error('At least one provider must be selected')
    );

    render(ProviderSelector);

    const checkbox = (await screen.findByTestId(
      'provider-checkbox-ChatGPT'
    )) as HTMLInputElement;

    await fireEvent.click(checkbox);

    expect(
      await screen.findByText(/At least one provider must be selected/i)
    ).toBeInTheDocument();

    // Checkbox should be reverted
    expect(checkbox.checked).toBe(false);
  });

  it('displays selected provider count', async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue([
      { ...mockProviders[0], is_selected: true },
      { ...mockProviders[1], is_selected: true },
      mockProviders[2],
    ]);

    render(ProviderSelector);

    expect(await screen.findByText(/Selected: 2 \/ 3/)).toBeInTheDocument();
  });

  it('displays auth badge for unauthenticated providers', async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue(mockProviders);

    render(ProviderSelector);

    // Claude is not authenticated in mock data
    const claudeItem = (await screen.findByText('Claude')).closest('label');
    expect(claudeItem?.querySelector('.auth-badge')).toBeInTheDocument();
  });
});
