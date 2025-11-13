<script lang="ts">
  import ProviderSelector from '../components/ProviderSelector.svelte';
</script>

<main class="container">
  <h1>ChenChen - Multi-LLM Prompt</h1>
  <p class="subtitle">Send prompts to multiple LLMs simultaneously</p>

  <ProviderSelector />

  <!-- TODO: Add PromptInput component -->
  <!-- TODO: Add ProviderPanel component for split-screen display -->
  <!-- TODO: Add StatusDisplay component -->
</main>

<style>
  :global(body) {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #0f0f0f;
    background-color: #f6f6f6;
    margin: 0;
    padding: 0;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    text-align: center;
    font-size: 2rem;
    margin-bottom: 0.5rem;
    color: #333;
  }

  .subtitle {
    text-align: center;
    color: #666;
    margin-bottom: 2rem;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    h1 {
      color: #f6f6f6;
    }

    .subtitle {
      color: #ccc;
    }
  }
</style>
