<script>
  import { invoke } from "@tauri-apps/api/core";

  /** @type {any[]} */
  let history = $state([]);
  let searchQuery = $state("");
  /** @type {string | null} */
  let expandedId = $state(null);
  let loading = $state(true);

  async function loadHistory() {
    loading = true;
    try {
      history = await invoke("get_history");
    } catch (err) {
      console.error("Failed to load history:", err);
    } finally {
      loading = false;
    }
  }

  /**
   * @param {string} query
   */
  async function handleSearch(query) {
    if (!query.trim()) {
      await loadHistory();
      return;
    }

    try {
      history = await invoke("search_history", { query: query.trim() });
    } catch (err) {
      console.error("Failed to search history:", err);
    }
  }

  async function handleClear() {
    if (!confirm("Are you sure you want to clear all history? This cannot be undone.")) {
      return;
    }

    try {
      await invoke("clear_history");
      history = [];
    } catch (err) {
      console.error("Failed to clear history:", err);
      alert(`Failed to clear history: ${err}`);
    }
  }

  /**
   * @param {string} text
   */
  async function copyToClipboard(text) {
    try {
      await navigator.clipboard.writeText(text);
      alert("Copied to clipboard!");
    } catch (err) {
      console.error("Failed to copy:", err);
      alert("Failed to copy to clipboard");
    }
  }

  /**
   * @param {string} id
   */
  function toggleExpand(id) {
    expandedId = expandedId === id ? null : id;
  }

  /**
   * @param {string} timestamp
   */
  function formatTime(timestamp) {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 0) return `${days}d ago`;
    if (hours > 0) return `${hours}h ago`;
    if (minutes > 0) return `${minutes}m ago`;
    return `${seconds}s ago`;
  }

  // Search with debounce
  /** @type {ReturnType<typeof setTimeout> | undefined} */
  let searchTimeout;
  $effect(() => {
    if (searchTimeout) clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      handleSearch(searchQuery);
    }, 300);
  });

  // Load history on mount
  loadHistory();
</script>

<main class="container">
  <div class="header">
    <h1>History</h1>
    <a href="/" class="back-link">← Back to Home</a>
  </div>

  <p class="subtitle">View your recent prompt processing history (last 100 entries).</p>

  <div class="controls">
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Search by template name or content..."
      class="search-input"
    />
    <button onclick={handleClear} disabled={history.length === 0} class="btn btn-danger">
      Clear All
    </button>
  </div>

  {#if loading}
    <div class="loading">Loading history...</div>
  {:else if history.length === 0}
    <div class="empty-state">
      <p>No history entries yet.</p>
      <p class="hint">History will appear here after you process text using hotkeys.</p>
    </div>
  {:else}
    <div class="history-list">
      {#each history as entry (entry.timestamp)}
        <div class="history-item">
          <div class="history-header">
            <div class="history-info">
              <span class="template-name">{entry.template_name}</span>
              <span class="timestamp">{formatTime(entry.timestamp)}</span>
            </div>
            <button
              class="expand-btn"
              onclick={() => toggleExpand(entry.timestamp)}
              aria-label={expandedId === entry.timestamp ? "Collapse" : "Expand"}
            >
              {expandedId === entry.timestamp ? "−" : "+"}
            </button>
          </div>

          <div class="preview-section">
            <div class="preview-item">
              <span class="preview-label">Input:</span>
              <span class="preview-text">{entry.source_preview}</span>
            </div>
            <div class="preview-item">
              <span class="preview-label">Output:</span>
              <span class="preview-text">{entry.result_preview}</span>
            </div>
          </div>

          {#if expandedId === entry.timestamp}
            <div class="expanded-content">
              <div class="full-content">
                <div class="content-header">
                  <strong>Full Input</strong>
                  <button class="copy-btn" onclick={() => copyToClipboard(entry.full_source)}>
                    Copy
                  </button>
                </div>
                <pre class="content-text">{entry.full_source}</pre>
              </div>

              <div class="full-content">
                <div class="content-header">
                  <strong>Full Output</strong>
                  <button class="copy-btn" onclick={() => copyToClipboard(entry.full_result)}>
                    Copy
                  </button>
                </div>
                <pre class="content-text">{entry.full_result}</pre>
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</main>

<style>
  .container {
    padding: 2rem;
    max-width: 1000px;
    margin: 0 auto;
    min-height: 100vh;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  h1 {
    margin: 0;
    font-size: 2rem;
  }

  .subtitle {
    color: #666;
    margin-bottom: 2rem;
    font-size: 1rem;
  }

  .back-link {
    color: #646cff;
    text-decoration: none;
    font-weight: 500;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    transition: background-color 0.2s;
  }

  .back-link:hover {
    background-color: rgba(100, 108, 255, 0.1);
  }

  .controls {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .search-input {
    flex: 1;
    padding: 0.6rem 1rem;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 0.95rem;
  }

  .search-input:focus {
    outline: none;
    border-color: #646cff;
  }

  .btn {
    padding: 0.6rem 1.2rem;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-danger {
    background: #ff4444;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #cc0000;
  }

  .loading, .empty-state {
    text-align: center;
    padding: 3rem;
    color: #666;
  }

  .empty-state .hint {
    font-size: 0.9rem;
    margin-top: 0.5rem;
  }

  .history-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .history-item {
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 1rem;
    background: #fff;
    transition: box-shadow 0.2s;
  }

  .history-item:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .history-info {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .template-name {
    font-weight: 600;
    color: #333;
  }

  .timestamp {
    font-size: 0.85rem;
    color: #999;
  }

  .expand-btn {
    background: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 4px;
    width: 32px;
    height: 32px;
    cursor: pointer;
    font-size: 1.2rem;
    font-weight: bold;
    transition: all 0.2s;
  }

  .expand-btn:hover {
    background: #e5e5e5;
  }

  .preview-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .preview-item {
    display: flex;
    gap: 0.5rem;
    font-size: 0.9rem;
  }

  .preview-label {
    font-weight: 500;
    color: #666;
    min-width: 60px;
  }

  .preview-text {
    color: #333;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .expanded-content {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #e0e0e0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .full-content {
    background: #f9f9f9;
    border-radius: 6px;
    padding: 1rem;
  }

  .content-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .content-header strong {
    font-size: 0.9rem;
    color: #666;
  }

  .copy-btn {
    background: #646cff;
    color: white;
    border: none;
    padding: 0.4rem 0.8rem;
    border-radius: 4px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .copy-btn:hover {
    background: #535ac8;
  }

  .content-text {
    margin: 0;
    padding: 0.75rem;
    background: #fff;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    white-space: pre-wrap;
    word-wrap: break-word;
    color: #333;
    max-height: 300px;
    overflow-y: auto;
  }

  @media (prefers-color-scheme: dark) {
    .subtitle {
      color: #aaa;
    }

    .search-input {
      background: #2a2a2a;
      border-color: #444;
      color: #fff;
    }

    .loading, .empty-state {
      color: #aaa;
    }

    .history-item {
      background: #1a1a1a;
      border-color: #444;
    }

    .template-name {
      color: #fff;
    }

    .timestamp {
      color: #777;
    }

    .expand-btn {
      background: #2a2a2a;
      border-color: #444;
      color: #fff;
    }

    .expand-btn:hover {
      background: #333;
    }

    .preview-label {
      color: #aaa;
    }

    .preview-text {
      color: #ddd;
    }

    .expanded-content {
      border-top-color: #333;
    }

    .full-content {
      background: #2a2a2a;
    }

    .content-header strong {
      color: #aaa;
    }

    .content-text {
      background: #1a1a1a;
      border-color: #444;
      color: #ddd;
    }
  }
</style>
