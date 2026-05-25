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
  let textareaElement = $state<HTMLTextAreaElement | null>(null);

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
    }, 3500);

    return () => clearInterval(interval);
  });

  // Auto-resize textarea heights
  $effect(() => {
    if (textareaElement) {
      // Force heights to recalculate
      textareaElement.style.height = 'auto';
      // Set scrollHeight with padding offset
      const newHeight = Math.min(220, Math.max(48, textareaElement.scrollHeight));
      textareaElement.style.height = `${newHeight}px`;
    }
  });

  // Handle prompt submission
  async function handleSubmit() {
    error = null;

    if (!prompt.trim()) {
      error = 'Please enter a prompt';
      return;
    }

    isVanishing = true;
    loading = true;

    try {
      const submissions = await tauri.submitPrompt(prompt);

      // Notify parent component with submissions
      onsubmitted?.(new CustomEvent('submitted', { detail: { submissions } }));

      // Clear input after animation
      setTimeout(() => {
        prompt = '';
        isVanishing = false;
        if (textareaElement) {
          textareaElement.style.height = '48px';
        }
      }, 300);
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to submit prompt';
      isVanishing = false;
    } finally {
      loading = false;
    }
  }

  // Handle keydown for submission (Enter submits, Shift+Enter newlines)
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSubmit();
    }
  }
</script>

<div class="prompt-container">
  <div class="input-wrapper-glow"></div>
  <div class="input-wrapper glass" class:vanishing={isVanishing}>
    <!-- Text area for multiline prompt support -->
    <textarea
      bind:this={textareaElement}
      bind:value={prompt}
      onkeydown={handleKeydown}
      placeholder={placeholders[currentPlaceholderIndex]}
      disabled={loading}
      data-testid="prompt-textarea"
      class:has-value={prompt.length > 0}
      rows="1"
      aria-label="Prompt inputs"
    ></textarea>

    <!-- UI feedback panel with helper information -->
    <div class="input-controls">
      <span class="hotkey-tip" class:visible={prompt.trim().length > 0}>
        <span>Shift + Enter for newline</span>
        <span class="badge secondary">⌘ ↵</span>
      </span>

      <!-- Glassmorphic Submit button -->
      <button
        onclick={handleSubmit}
        disabled={loading || !prompt.trim()}
        data-testid="submit-button"
        class="submit-button"
        class:active={prompt.trim().length > 0}
        aria-label="Submit prompt"
      >
        {#if loading}
          <svg class="spinner-loader" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="32" class="spinner-circle"></circle>
          </svg>
        {:else}
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M22 2L11 13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M22 2L15 22L11 13L2 9L22 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        {/if}
      </button>
    </div>
  </div>

  {#if error}
    <div class="error-message" role="alert">
      <svg class="error-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M12 9V14M12 17.01H12.01M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span>{error}</span>
    </div>
  {/if}
</div>

<style>
  .prompt-container {
    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;
    max-width: 850px;
    margin: 0 auto;
    width: 100%;
  }

  /* Focused ambient light effect */
  .input-wrapper-glow {
    position: absolute;
    top: -2px;
    left: -2px;
    right: -2px;
    bottom: -2px;
    background: linear-gradient(135deg, var(--primary-color), var(--accent-color));
    border-radius: var(--radius-lg);
    opacity: 0;
    filter: blur(12px);
    transition: opacity var(--transition-normal);
    z-index: 0;
    pointer-events: none;
  }

  .prompt-container:focus-within .input-wrapper-glow {
    opacity: 0.18;
  }

  /* Main Command Box */
  .input-wrapper {
    position: relative;
    display: flex;
    flex-direction: column;
    padding: 0.65rem 0.75rem 0.65rem 1.25rem;
    border-radius: var(--radius-lg);
    background: hsla(220, 15%, 8%, 0.75);
    border: 1px solid hsla(220, 20%, 100%, 0.06);
    box-shadow: var(--shadow-lg), 0 10px 30px rgba(0, 0, 0, 0.4);
    transition: all var(--transition-normal);
    z-index: 1;
  }

  .input-wrapper:focus-within {
    background: hsla(220, 15%, 11%, 0.9);
    border-color: hsla(220, 95%, 60%, 0.35);
    box-shadow: var(--shadow-lg), 0 15px 40px rgba(0, 0, 0, 0.5), 0 0 10px hsla(220, 95%, 60%, 0.1);
    transform: translateY(-2px);
  }

  .input-wrapper.vanishing {
    opacity: 0;
    transform: translateY(20px) scale(0.96);
    filter: blur(12px);
  }

  /* Auto-expanding Textarea */
  textarea {
    flex: 1;
    padding: 0.4rem 0.5rem 0.4rem 0;
    font-family: var(--font-family);
    font-size: 0.98rem;
    background: transparent;
    border: none;
    color: var(--text-primary);
    line-height: 1.5;
    resize: none;
    min-height: 48px;
    height: 48px;
    max-height: 220px;
    outline: none;
    box-shadow: none;
    transition: color 0.2s ease;
  }

  textarea::placeholder {
    color: var(--text-tertiary);
    font-weight: 400;
  }

  textarea:focus {
    box-shadow: none;
    border: none;
    background: transparent;
  }

  textarea:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  /* Controls Panel inside input */
  .input-controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: 0.5rem;
    border-top: 1px solid hsla(220, 20%, 100%, 0.03);
    margin-top: 0.25rem;
  }

  .hotkey-tip {
    font-size: 0.72rem;
    color: var(--text-tertiary);
    display: flex;
    align-items: center;
    gap: 0.4rem;
    opacity: 0;
    transform: translateX(-5px);
    transition: all var(--transition-normal);
    pointer-events: none;
  }

  .hotkey-tip.visible {
    opacity: 1;
    transform: translateX(0);
  }

  .hotkey-tip .badge {
    padding: 0.15rem 0.4rem;
    border-radius: 4px;
    font-size: 0.65rem;
  }

  /* Submit Action Button */
  .submit-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 38px;
    height: 38px;
    padding: 0;
    background: hsla(220, 15%, 15%, 0.8);
    border: 1px solid var(--border-highlight);
    border-radius: 50%;
    cursor: pointer;
    transition: all var(--transition-spring);
    flex-shrink: 0;
    color: var(--text-secondary);
    box-shadow: var(--shadow-sm);
    margin-left: auto;
  }

  .submit-button svg {
    width: 16px;
    height: 16px;
    transition: transform var(--transition-normal);
  }

  .submit-button.active {
    background: linear-gradient(135deg, var(--primary-color) 0%, hsl(220, 90%, 50%) 100%);
    border-color: transparent;
    color: white;
    box-shadow: 0 4px 12px var(--primary-glow);
  }

  .submit-button:hover:not(:disabled) {
    transform: scale(1.1) rotate(-8deg);
  }

  .submit-button.active:hover {
    box-shadow: 0 6px 16px hsla(220, 95%, 60%, 0.45);
  }

  .submit-button:active:not(:disabled) {
    transform: scale(0.95);
  }

  .submit-button:disabled {
    background: hsla(220, 15%, 12%, 0.4);
    color: var(--text-tertiary);
    cursor: not-allowed;
    box-shadow: none;
    transform: none;
    border-color: transparent;
  }

  /* Custom Spinner Loader */
  .spinner-loader {
    width: 18px;
    height: 18px;
    animation: spin-pulsing 1.2s linear infinite;
  }

  .spinner-circle {
    stroke-dasharray: 48;
    stroke-dashoffset: 24;
    stroke-linecap: round;
    color: white;
  }

  /* Popover styled error overlay */
  .error-message {
    position: absolute;
    bottom: calc(100% + 12px);
    left: 0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1.1rem;
    background: hsla(355, 80%, 20%, 0.85);
    backdrop-filter: blur(12px);
    border: 1px solid hsla(355, 85%, 55%, 0.25);
    border-radius: var(--radius-md);
    font-size: 0.85rem;
    color: hsl(355, 95%, 85%);
    animation: error-slide 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    box-shadow: var(--shadow-lg), 0 5px 15px rgba(0, 0, 0, 0.3);
    z-index: 1000;
  }

  .error-icon {
    width: 16px;
    height: 16px;
    color: var(--error-color);
    flex-shrink: 0;
  }

  @keyframes error-slide {
    from {
      opacity: 0;
      transform: translateY(12px) scale(0.96);
      filter: blur(4px);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
      filter: blur(0);
    }
  }
</style>
