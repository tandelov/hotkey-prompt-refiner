<script>
  import { invoke } from "@tauri-apps/api/core";

  let apiKey = $state("");
  let savedKey = $state("");
  let showKey = $state(false);
  let status = $state("idle"); // idle, saving, testing, success, error
  let message = $state("");

  async function loadApiKey() {
    try {
      const key = await invoke("get_api_key");
      if (key) {
        savedKey = key;
        apiKey = key;
      }
    } catch (err) {
      console.error("Failed to load API key:", err);
    }
  }

  async function saveApiKey() {
    if (!apiKey.trim()) {
      status = "error";
      message = "API key cannot be empty";
      return;
    }

    status = "saving";
    message = "";

    try {
      await invoke("save_api_key", { apiKey: apiKey.trim() });
      savedKey = apiKey.trim();
      status = "success";
      message = "API key saved successfully";

      setTimeout(() => {
        if (status === "success") {
          status = "idle";
          message = "";
        }
      }, 3000);
    } catch (err) {
      status = "error";
      message = `Failed to save: ${err}`;
    }
  }

  async function testApiKey() {
    const keyToTest = apiKey.trim() || savedKey;

    if (!keyToTest) {
      status = "error";
      message = "No API key to test";
      return;
    }

    status = "testing";
    message = "Testing API key...";

    try {
      const isValid = await invoke("test_api_key", { apiKey: keyToTest });

      if (isValid) {
        status = "success";
        message = "✓ API key is valid";
      } else {
        status = "error";
        message = "API key is invalid";
      }

      setTimeout(() => {
        if (status === "success" || status === "error") {
          status = "idle";
          message = "";
        }
      }, 3000);
    } catch (err) {
      status = "error";
      message = `Test failed: ${err}`;
    }
  }

  async function deleteApiKey() {
    if (!confirm("Are you sure you want to delete the API key?")) {
      return;
    }

    try {
      await invoke("delete_api_key");
      apiKey = "";
      savedKey = "";
      status = "success";
      message = "API key deleted";

      setTimeout(() => {
        status = "idle";
        message = "";
      }, 2000);
    } catch (err) {
      status = "error";
      message = `Failed to delete: ${err}`;
    }
  }

  // Load API key on mount
  loadApiKey();
</script>

<div class="api-key-section">
  <h2>Anthropic API Key</h2>
  <p class="description">
    Your API key is stored securely in the system keychain.
    Get your key from <a href="https://console.anthropic.com/settings/keys" target="_blank">Anthropic Console</a>.
  </p>

  <div class="input-group">
    <div class="input-wrapper">
      <input
        type={showKey ? "text" : "password"}
        bind:value={apiKey}
        placeholder="sk-ant-..."
        class="api-key-input"
        disabled={status === "saving" || status === "testing"}
      />
      <button
        class="toggle-visibility"
        onclick={() => showKey = !showKey}
        type="button"
        title={showKey ? "Hide key" : "Show key"}
      >
        {showKey ? "🙈" : "👁️"}
      </button>
    </div>

    <div class="button-group">
      <button
        onclick={saveApiKey}
        disabled={status === "saving" || status === "testing" || !apiKey.trim()}
        class="btn btn-primary"
      >
        {status === "saving" ? "Saving..." : "Save"}
      </button>

      <button
        onclick={testApiKey}
        disabled={status === "saving" || status === "testing" || (!apiKey.trim() && !savedKey)}
        class="btn btn-secondary"
      >
        {status === "testing" ? "Testing..." : "Test"}
      </button>

      <button
        onclick={deleteApiKey}
        disabled={status === "saving" || status === "testing" || !savedKey}
        class="btn btn-danger"
      >
        Delete
      </button>
    </div>
  </div>

  {#if message}
    <div class="message" class:success={status === "success"} class:error={status === "error"}>
      {message}
    </div>
  {/if}
</div>

<style>
  .api-key-section {
    margin-bottom: 2rem;
    padding: 1.5rem;
    border: 1px solid #ddd;
    border-radius: 8px;
    background: #fff;
  }

  h2 {
    margin-top: 0;
    margin-bottom: 0.5rem;
    font-size: 1.25rem;
  }

  .description {
    color: #666;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .description a {
    color: #646cff;
    text-decoration: none;
  }

  .description a:hover {
    text-decoration: underline;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .input-wrapper {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .api-key-input {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 8px;
    font-family: 'Courier New', monospace;
    font-size: 14px;
    color: #1d1d1f;
  }

  .api-key-input:focus {
    outline: none;
    border-color: #007AFF;
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.3);
  }

  .api-key-input:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }

  .toggle-visibility {
    padding: 0.6rem;
    background: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 6px;
    cursor: pointer;
    font-size: 1.2rem;
    transition: background-color 0.2s;
  }

  .toggle-visibility:hover {
    background: #e5e5e5;
  }

  .button-group {
    display: flex;
    gap: 0.5rem;
  }

  .btn {
    padding: 8px 16px;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #007AFF;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #0051D5;
  }

  .btn-secondary {
    background: rgba(0, 0, 0, 0.08);
    color: #1d1d1f;
    border: none;
  }

  .btn-secondary:hover:not(:disabled) {
    background: rgba(0, 0, 0, 0.12);
  }

  .btn-danger {
    background: transparent;
    color: #FF453A;
  }

  .btn-danger:hover:not(:disabled) {
    background: rgba(255, 69, 58, 0.15);
  }

  .message {
    margin-top: 0.75rem;
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.9rem;
  }

  .message.success {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
  }

  .message.error {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
  }

  @media (prefers-color-scheme: dark) {
    .api-key-section {
      background: #1a1a1a;
      border-color: #444;
    }

    .description {
      color: #aaa;
    }

    .api-key-input {
      background: rgba(255, 255, 255, 0.1);
      border: 1px solid rgba(255, 255, 255, 0.15);
      color: white;
    }

    .api-key-input:focus {
      border-color: #007AFF;
      box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.3);
    }

    .api-key-input:disabled {
      background-color: rgba(255, 255, 255, 0.05);
    }

    .toggle-visibility {
      background: rgba(255, 255, 255, 0.1);
      border-color: rgba(255, 255, 255, 0.15);
      color: #fff;
    }

    .toggle-visibility:hover {
      background: rgba(255, 255, 255, 0.15);
    }

    .btn-secondary {
      background: rgba(255, 255, 255, 0.15);
      color: white;
    }

    .btn-secondary:hover:not(:disabled) {
      background: rgba(255, 255, 255, 0.2);
    }
  }
</style>
