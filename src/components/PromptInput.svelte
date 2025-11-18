<script lang="ts">
  import { onMount } from 'svelte';
  import { tauri } from '../services/tauri';
  import type { Submission } from '../types';

  // Component state
  let prompt = $state('');
  let loading = $state(false);
  let error = $state<string | null>(null);
  let currentPlaceholderIndex = $state(0);
  let isVanishing = $state(false);

  // Cycling placeholders for modern effect
  const placeholders = [
    'Ask me anything...',
    'Compare responses from multiple AIs...',
    'What would you like to know?',
    'Enter your prompt here...',
    'Try asking a creative question...',
  ];

  // Props for event handling (Svelte 5 pattern)
  interface Props {
    onsubmitted?: (event: CustomEvent<{ submissions: Submission[] }>) => void;
  }
  let { onsubmitted }: Props = $props();

  // Cycle through placeholders
  onMount(() => {
    const interval = setInterval(() => {
      currentPlaceholderIndex = (currentPlaceholderIndex + 1) % placeholders.length;
    }, 3000);

    return () => clearInterval(interval);
  });

  // T118: Handle prompt submission
  async function handleSubmit() {
    error = null;

    // T117: Validate non-empty prompt
    if (!prompt.trim()) {
      error = 'Please enter a prompt';
      return;
    }

    // Trigger vanish animation
    isVanishing = true;
    loading = true;

    try {
      // T118: Call submit_prompt command
      const submissions = await tauri.submitPrompt(prompt);

      // Notify parent component with submissions
      onsubmitted?.(new CustomEvent('submitted', { detail: { submissions } }));

      // Clear input after animation
      setTimeout(() => {
        prompt = '';
        isVanishing = false;
      }, 300);
    } catch (e) {
      // T119: Display validation error
      error = e instanceof Error ? e.message : 'Failed to submit prompt';
      isVanishing = false;
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

<div class="prompt-container">
  <div class="input-wrapper" class:vanishing={isVanishing}>
    <input
      type="text"
      bind:value={prompt}
      onkeydown={handleKeydown}
      placeholder={placeholders[currentPlaceholderIndex]}
      disabled={loading}
      data-testid="prompt-textarea"
      class:has-value={prompt.length > 0}
    />

    <button
      onclick={handleSubmit}
      disabled={loading || !prompt.trim()}
      data-testid="submit-button"
      class="submit-button"
      aria-label="Submit prompt"
    >
      {#if loading}
        <svg class="spinner" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
      {:else}
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M22 2L11 13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M22 2L15 22L11 13L2 9L22 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      {/if}
    </button>
  </div>

  {#if error}
    <div class="error-message">{error}</div>
  {/if}
</div>

<style>
  .prompt-container {
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: rgba(30, 30, 30, 0.8);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .input-wrapper:focus-within {
    background: rgba(30, 30, 30, 0.95);
    border-color: rgba(102, 126, 234, 0.5);
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.15),
                0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .input-wrapper.vanishing {
    opacity: 0;
    transform: translateY(-10px) scale(0.98);
  }

  input {
    flex: 1;
    padding: 0.75rem 1rem;
    font-family: inherit;
    font-size: 0.95rem;
    background: transparent;
    border: none;
    color: #f6f6f6;
    transition: all 0.3s ease;
  }

  input::placeholder {
    color: #666;
    transition: color 0.3s ease, opacity 0.3s ease;
    animation: placeholder-slide 0.5s ease;
  }

  @keyframes placeholder-slide {
    0% {
      opacity: 0;
      transform: translateY(-5px);
    }
    100% {
      opacity: 1;
      transform: translateY(0);
    }
  }

  input:focus {
    outline: none;
  }

  input:focus::placeholder {
    color: #555;
  }

  input:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  .submit-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    padding: 0;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    flex-shrink: 0;
    color: white;
  }

  .submit-button svg {
    width: 20px;
    height: 20px;
    transition: transform 0.2s ease;
  }

  .submit-button:hover:not(:disabled) {
    transform: scale(1.05);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  }

  .submit-button:active:not(:disabled) {
    transform: scale(0.95);
  }

  .submit-button:disabled {
    background: linear-gradient(135deg, #ccc 0%, #999 100%);
    cursor: not-allowed;
    opacity: 0.5;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .error-message {
    position: absolute;
    bottom: -1.5rem;
    left: 0;
    font-size: 0.8rem;
    color: #ef4444;
    animation: error-slide 0.3s ease;
  }

  @keyframes error-slide {
    from {
      opacity: 0;
      transform: translateY(-5px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

</style>
