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

<div class="prompt-input-container">
  <h2>Enter Your Prompt</h2>

  <div class="input-wrapper">
    <!-- T117: Textarea with prompt binding -->
    <textarea
      bind:value={prompt}
      on:keydown={handleKeydown}
      placeholder="Enter your prompt here... (Press Enter to send, Shift+Enter for new line)"
      rows="4"
      disabled={loading}
      data-testid="prompt-textarea"
    />

    <!-- T118: Send button -->
    <button
      on:click={handleSubmit}
      disabled={loading || !prompt.trim()}
      class="submit-button"
      data-testid="submit-button"
    >
      {loading ? 'Sending...' : 'Send to Selected Providers'}
    </button>
  </div>

  <!-- T119: Error display -->
  {#if error}
    <div class="error" role="alert" data-testid="prompt-error">
      <strong>Error:</strong>
      {error}
    </div>
  {/if}

  <p class="hint">
    Your prompt will be sent to all selected LLM providers simultaneously.
  </p>
</div>

<style>
  .prompt-input-container {
    padding: 1.5rem;
    background: var(--bg-primary, white);
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    margin: 1rem 0;
  }

  h2 {
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
    color: var(--text-primary, #333);
  }

  .input-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  textarea {
    width: 100%;
    padding: 0.75rem;
    font-family: inherit;
    font-size: 1rem;
    border: 2px solid var(--border-color, #ddd);
    border-radius: 6px;
    resize: vertical;
    transition: border-color 0.2s;
  }

  textarea:focus {
    outline: none;
    border-color: var(--primary-color, #4a9eff);
  }

  textarea:disabled {
    background: var(--bg-disabled, #f5f5f5);
    cursor: not-allowed;
  }

  .submit-button {
    align-self: flex-end;
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    font-weight: 600;
    color: white;
    background: var(--primary-color, #4a9eff);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .submit-button:hover:not(:disabled) {
    background: var(--primary-color-dark, #3a8eef);
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(74, 158, 255, 0.3);
  }

  .submit-button:disabled {
    background: var(--bg-disabled, #ccc);
    cursor: not-allowed;
    transform: none;
  }

  .error {
    padding: 0.75rem;
    background: var(--error-bg, #fee);
    border: 1px solid var(--error-border, #fcc);
    border-radius: 4px;
    color: var(--error-text, #c33);
    margin-top: 0.75rem;
  }

  .hint {
    margin-top: 0.75rem;
    font-size: 0.85rem;
    color: var(--text-secondary, #666);
    font-style: italic;
  }
</style>
