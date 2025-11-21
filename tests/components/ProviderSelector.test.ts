import '@testing-library/jest-dom/vitest';
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, waitFor } from "@testing-library/svelte";
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

// Mock Tauri API menu modules
vi.mock('@tauri-apps/api/menu', () => ({
  Menu: {
    new: vi.fn(() => ({
      append: vi.fn(),
      popup: vi.fn(),
    })),
  },
  CheckMenuItem: {
    new: vi.fn(),
  },
}));

vi.mock('@tauri-apps/api/window', () => ({
  LogicalPosition: vi.fn(),
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

  it("displays button with default text when no providers are selected", async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue(mockProviders);

    render(ProviderSelector);

    // Wait for the button to appear (loading should be false)
    const button = await screen.findByTestId("provider-selector-button");
    expect(button).toBeInTheDocument();

    // Should show "Select LLMs" when no providers are selected
    expect(screen.getByText("Select LLMs")).toBeInTheDocument();
  });

  it("displays selected provider names when providers are selected", async () => {
    const selectedProviders = [
      { ...mockProviders[0], is_selected: true },
      { ...mockProviders[1], is_selected: false },
      { ...mockProviders[2], is_selected: false },
    ];
    vi.mocked(tauri.getProviders).mockResolvedValue(selectedProviders);

    render(ProviderSelector);

    const button = await screen.findByTestId("provider-selector-button");
    expect(button).toBeInTheDocument();

    // Should show the selected provider name
    expect(screen.getByText("ChatGPT")).toBeInTheDocument();

    // Should show count badge
    expect(screen.getByText("1")).toBeInTheDocument();
  });

  it('displays "All LLMs" when all providers are selected', async () => {
    const allSelectedProviders = mockProviders.map((p) => ({
      ...p,
      is_selected: true,
    }));
    vi.mocked(tauri.getProviders).mockResolvedValue(allSelectedProviders);

    render(ProviderSelector);

    const button = await screen.findByTestId("provider-selector-button");
    expect(button).toBeInTheDocument();

    // Should show "All LLMs" when all are selected
    expect(screen.getByText("All LLMs")).toBeInTheDocument();

    // Should show count badge with total count
    expect(screen.getByText("3")).toBeInTheDocument();
  });

  it("displays comma-separated names when multiple providers are selected", async () => {
    const multipleSelected = [
      { ...mockProviders[0], is_selected: true },
      { ...mockProviders[1], is_selected: true },
      { ...mockProviders[2], is_selected: false },
    ];
    vi.mocked(tauri.getProviders).mockResolvedValue(multipleSelected);

    render(ProviderSelector);

    const button = await screen.findByTestId("provider-selector-button");
    expect(button).toBeInTheDocument();

    // Should show comma-separated names
    expect(screen.getByText("ChatGPT, Gemini")).toBeInTheDocument();

    // Should show count badge
    expect(screen.getByText("2")).toBeInTheDocument();
  });

  it("loads providers on mount", async () => {
    vi.mocked(tauri.getProviders).mockResolvedValue(mockProviders);

    render(ProviderSelector);

    // Wait for providers to load
    await waitFor(() => {
      expect(tauri.getProviders).toHaveBeenCalled();
    });

    const button = await screen.findByTestId("provider-selector-button");
    expect(button).toBeInTheDocument();
  });

  it("handles loading errors gracefully", async () => {
    const consoleError = vi
      .spyOn(console, "error")
      .mockImplementation(() => {});
    vi.mocked(tauri.getProviders).mockRejectedValue(
      new Error("Failed to load providers")
    );

    render(ProviderSelector);

    // Wait for the error to be logged
    await waitFor(() => {
      expect(consoleError).toHaveBeenCalledWith(
        "Failed to load providers:",
        expect.any(Error)
      );
    });

    consoleError.mockRestore();
  });
});
