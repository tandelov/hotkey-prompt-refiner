<script>
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");
  /** @type {any[]} */
  let templates = $state([]);
  let activeHotkeys = $state(0);

  async function greet(/** @type {SubmitEvent} */ event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  async function loadStatus() {
    try {
      templates = await invoke("get_templates");
      activeHotkeys = templates.filter(t => t.hotkey).length;
    } catch (err) {
      console.error("Failed to load status:", err);
    }
  }

  // Load status on mount
  loadStatus();
</script>

<main class="container">
  <h1>Hotkey Prompt Refiner</h1>

  <nav class="nav">
    <a href="/settings" class="nav-link">Settings</a>
    <a href="/history" class="nav-link">History</a>
  </nav>

  <div class="info">
    <p>AI-powered text processing via global hotkeys</p>
    <p class="hint">💡 Tip: Use the system tray icon to quickly access Settings and History</p>
  </div>

  <div class="status-section">
    <h2>Status</h2>
    <div class="status-card">
      <div class="status-item">
        <span class="status-label">Templates:</span>
        <span class="status-value">{templates.length}</span>
      </div>
      <div class="status-item">
        <span class="status-label">Active Hotkeys:</span>
        <span class="status-value {activeHotkeys > 0 ? 'active' : 'inactive'}">
          {activeHotkeys}
        </span>
      </div>
      {#if activeHotkeys === 0}
        <div class="status-hint">
          No hotkeys configured. <a href="/settings">Go to Settings</a> to create templates.
        </div>
      {:else}
        <div class="status-hint success">
          ✓ Hotkeys are active! Copy text and press your configured hotkey.
        </div>
      {/if}
    </div>
  </div>

  <div class="demo-section">
    <h2>Demo</h2>
    <form class="row" onsubmit={greet}>
      <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
      <button type="submit">Greet</button>
    </form>
    <p>{greetMsg}</p>
  </div>
</main>

<style>
.nav {
  display: flex;
  gap: 1.5rem;
  margin: 2rem 0;
}

.nav-link {
  font-weight: 500;
  color: #646cff;
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  transition: background-color 0.2s;
}

.nav-link:hover {
  background-color: rgba(100, 108, 255, 0.1);
}

.info {
  margin: 1rem 0;
  color: #666;
}

.status-section {
  width: 100%;
  margin: 2rem 0;
}

.status-section h2 {
  margin-bottom: 1rem;
}

.status-card {
  padding: 1.5rem;
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.status-item {
  display: flex;
  justify-content: space-between;
  padding: 0.75rem 0;
  border-bottom: 1px solid #e0e0e0;
}

.status-item:last-of-type {
  border-bottom: none;
}

.status-label {
  font-weight: 500;
  color: #666;
}

.status-value {
  font-weight: 600;
  font-size: 1.1rem;
}

.status-value.active {
  color: #4caf50;
}

.status-value.inactive {
  color: #999;
}

.status-hint {
  margin-top: 1rem;
  padding: 0.75rem;
  background: #f0f0f0;
  border-radius: 6px;
  font-size: 0.9rem;
  text-align: center;
  color: #666;
}

.status-hint.success {
  background: #e8f5e9;
  color: #2e7d32;
}

.status-hint a {
  color: #646cff;
  text-decoration: none;
  font-weight: 500;
}

.status-hint a:hover {
  text-decoration: underline;
}

.demo-section {
  width: 100%;
  margin-top: 3rem;
  padding: 2rem;
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0 auto;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  max-width: 800px;
}

.row {
  display: flex;
  justify-content: center;
}

h1 {
  text-align: center;
  margin-bottom: 0.5rem;
}

h2 {
  margin-bottom: 1rem;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .nav-link:hover {
    background-color: rgba(100, 108, 255, 0.2);
  }

  .info {
    color: #aaa;
  }

  .info .hint {
    font-size: 0.9rem;
    margin-top: 0.5rem;
    opacity: 0.8;
  }

  .status-card {
    background: #1a1a1a;
  }

  .status-item {
    border-bottom-color: #333;
  }

  .status-label {
    color: #aaa;
  }

  .status-hint {
    background: #2a2a2a;
    color: #aaa;
  }

  .status-hint.success {
    background: #1e4620;
    color: #81c784;
  }

  .demo-section {
    background: #1a1a1a;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
