<script>
  import { invoke } from "@tauri-apps/api/core";

  /** @type {any[]} */
  let history = $state([]);
  let searchQuery = $state("");
  /** @type {any | null} */
  let selectedEntry = $state(null);
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
    if (!confirm("Clear all history?")) {
      return;
    }

    try {
      await invoke("clear_history");
      history = [];
      selectedEntry = null;
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
    } catch (err) {
      console.error("Failed to copy:", err);
    }
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
    return `Just now`;
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

  loadHistory();
</script>

<div class="history-page">
  <div class="page-header">
    <h1>History</h1>
    <button class="btn-clear" onclick={handleClear} disabled={history.length === 0}>Clear</button>
  </div>

  <div class="content-layout">
    <div class="list-panel">
      <div class="search-bar">
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search..."
          class="search-input"
        />
      </div>

      <div class="history-list">
        {#if loading}
          <div class="empty-message">Loading...</div>
        {:else if history.length === 0}
          <div class="empty-message">No history</div>
        {:else}
          {#each history as entry (entry.timestamp)}
            <button
              class="history-row"
              class:selected={selectedEntry?.timestamp === entry.timestamp}
              onclick={() => selectedEntry = entry}
            >
              <div class="row-header">
                <span class="row-template">{entry.template_name}</span>
                <span class="row-time">{formatTime(entry.timestamp)}</span>
              </div>
              <div class="row-preview">{entry.source_preview}</div>
            </button>
          {/each}
        {/if}
      </div>
    </div>

    <div class="detail-panel">
      {#if selectedEntry}
        <div class="detail-content">
          <div class="detail-section">
            <div class="detail-header">
              <span class="detail-title">Input</span>
              <button class="btn-copy" onclick={() => copyToClipboard(selectedEntry.full_source)}>Copy</button>
            </div>
            <div class="detail-text">{selectedEntry.full_source}</div>
          </div>

          <div class="detail-section">
            <div class="detail-header">
              <span class="detail-title">Output</span>
              <button class="btn-copy" onclick={() => copyToClipboard(selectedEntry.full_result)}>Copy</button>
            </div>
            <div class="detail-text">{selectedEntry.full_result}</div>
          </div>
        </div>
      {:else}
        <div class="empty-detail">
          <span>Select a history entry</span>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .history-page {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .page-header {
    padding: 20px 24px 16px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    flex-shrink: 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .page-header h1 {
    margin: 0;
    font-size: 22px;
    font-weight: 600;
    color: #1d1d1f;
  }

  .btn-clear {
    padding: 5px 14px;
    border: none;
    border-radius: 6px;
    background: #ff3b30;
    color: white;
    font-size: 13px;
    font-family: inherit;
    cursor: pointer;
  }

  .btn-clear:hover:not(:disabled) {
    background: #d62518;
  }

  .btn-clear:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .content-layout {
    flex: 1;
    display: flex;
    min-height: 0;
  }

  .list-panel {
    width: 260px;
    border-right: 1px solid rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .search-bar {
    padding: 12px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  }

  .search-input {
    width: 100%;
    padding: 6px 10px;
    border: 1px solid rgba(0, 0, 0, 0.2);
    border-radius: 6px;
    font-size: 13px;
    font-family: inherit;
    background: #ffffff;
    color: #1d1d1f;
  }

  .search-input:focus {
    outline: none;
    border-color: #007aff;
  }

  .search-input::placeholder {
    color: #86868b;
  }

  .history-list {
    flex: 1;
    overflow-y: auto;
  }

  .empty-message {
    padding: 40px 20px;
    text-align: center;
    font-size: 13px;
    color: #86868b;
  }

  .history-row {
    width: 100%;
    padding: 10px 12px;
    border: none;
    border-bottom: 1px solid rgba(0, 0, 0, 0.08);
    background: transparent;
    text-align: left;
    cursor: pointer;
    font-family: inherit;
  }

  .history-row:hover {
    background: rgba(0, 0, 0, 0.03);
  }

  .history-row.selected {
    background: rgba(0, 122, 255, 0.15);
  }

  .row-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
  }

  .row-template {
    font-size: 13px;
    font-weight: 500;
    color: #1d1d1f;
  }

  .row-time {
    font-size: 11px;
    color: #86868b;
  }

  .row-preview {
    font-size: 12px;
    color: #86868b;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .detail-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .detail-content {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .detail-section {
    flex-shrink: 0;
  }

  .detail-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .detail-title {
    font-size: 14px;
    font-weight: 600;
    color: #1d1d1f;
  }

  .btn-copy {
    padding: 4px 12px;
    border: none;
    border-radius: 6px;
    background: rgba(0, 122, 255, 0.1);
    color: #007aff;
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
  }

  .btn-copy:hover {
    background: rgba(0, 122, 255, 0.2);
  }

  .detail-text {
    padding: 12px;
    background: rgba(0, 0, 0, 0.03);
    border-radius: 8px;
    font-size: 13px;
    line-height: 1.6;
    color: #1d1d1f;
    white-space: pre-wrap;
    word-wrap: break-word;
    max-height: 200px;
    overflow-y: auto;
  }

  .empty-detail {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .empty-detail span {
    font-size: 13px;
    color: #86868b;
  }

  @media (prefers-color-scheme: dark) {
    .page-header {
      border-bottom-color: rgba(255, 255, 255, 0.1);
    }

    .page-header h1,
    .row-template,
    .detail-title,
    .detail-text {
      color: #f5f5f7;
    }

    .list-panel {
      border-right-color: rgba(255, 255, 255, 0.1);
    }

    .search-bar {
      border-bottom-color: rgba(255, 255, 255, 0.1);
    }

    .search-input {
      background: rgba(255, 255, 255, 0.1);
      border-color: rgba(255, 255, 255, 0.2);
      color: #f5f5f7;
    }

    .history-row {
      border-bottom-color: rgba(255, 255, 255, 0.08);
    }

    .history-row:hover {
      background: rgba(255, 255, 255, 0.05);
    }

    .history-row.selected {
      background: rgba(10, 132, 255, 0.25);
    }

    .detail-text {
      background: rgba(255, 255, 255, 0.05);
    }

    .btn-copy {
      background: rgba(10, 132, 255, 0.2);
      color: #0a84ff;
    }

    .btn-copy:hover {
      background: rgba(10, 132, 255, 0.3);
    }
  }
</style>
