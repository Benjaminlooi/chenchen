<script lang="ts">
  import { tauri } from '../services/tauri';
  import type { Submission } from '../types';

  // Component state
  let prompt = '';
  let loading = false;
  let error: string | null = null;

  // Event to notify parent when prompt is submitted
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher<{
    submitted: { submissions: Submission[] };
  }>();

  // T118: Handle prompt submission
  async function handleSubmit() {
    error = null;

    // T117: Validate non-empty prompt
    if (!prompt.trim()) {
      error = 'Please enter a prompt';
      return;
    }

    loading = true;

    try {
      // T118: Call submit_prompt command
      const submissions = await tauri.submitPrompt(prompt);

      // Notify parent component with submissions
      dispatch('submitted', { submissions });

      // Clear input after successful submission
      prompt = '';
    } catch (e) {
      // T119: Display validation error
      error = e instanceof Error ? e.message : 'Failed to submit prompt';
    } finally {
      loading = false;
    }
  }

  // Handle Enter key (with Shift for newline)
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSubmit();
    }
  }
</script>

<div class="prompt-input">
  <input
    type="text"
    bind:value={prompt}
    on:keydown={handleKeydown}
    placeholder="Enter your prompt..."
    disabled={loading}
    data-testid="prompt-textarea"
  />
  <button
    on:click={handleSubmit}
    disabled={loading || !prompt.trim()}
    data-testid="submit-button"
  >
    {loading ? 'Sending...' : 'Send'}
  </button>
</div>

<style>
  .prompt-input {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex: 1;
  }

  input {
    flex: 1;
    padding: 0.5rem 0.75rem;
    font-family: inherit;
    font-size: 0.95rem;
    border: 1px solid var(--border-color, #ddd);
    border-radius: 4px;
    transition: border-color 0.2s;
  }

  input:focus {
    outline: none;
    border-color: var(--primary-color, #4a9eff);
  }

  input:disabled {
    background: var(--bg-disabled, #f5f5f5);
    cursor: not-allowed;
  }

  button {
    padding: 0.5rem 1.5rem;
    font-size: 0.95rem;
    font-weight: 600;
    color: white;
    background: var(--primary-color, #4a9eff);
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  button:hover:not(:disabled) {
    background: var(--primary-color-dark, #3a8eef);
  }

  button:disabled {
    background: var(--bg-disabled, #ccc);
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>
