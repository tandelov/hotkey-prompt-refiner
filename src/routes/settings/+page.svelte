<script>
  import ApiKeyInput from '$lib/components/ApiKeyInput.svelte';
  import TemplateEditor from '$lib/components/TemplateEditor.svelte';
  import { invoke } from "@tauri-apps/api/core";

  /** @type {any[]} */
  let templates = $state([]);
  let defaultModel = $state("claude-3-5-haiku-20241022");
  let showEditor = $state(false);
  /** @type {any} */
  let editingTemplate = $state(null);
  /** @type {ReturnType<typeof setTimeout> | null} */
  let modelChangeTimeout = null;
  let autoLaunchEnabled = $state(false);
  let autoLaunchLoading = $state(false);

  async function loadSettings() {
    try {
      templates = await invoke("get_templates");
      defaultModel = await invoke("get_default_model");
      autoLaunchEnabled = await invoke("is_autostart_enabled");
    } catch (err) {
      console.error("Failed to load settings:", err);
    }
  }

  async function handleModelChange() {
    // Debounce model changes to avoid too many API calls
    if (modelChangeTimeout) {
      clearTimeout(modelChangeTimeout);
    }

    modelChangeTimeout = setTimeout(async () => {
      try {
        await invoke("set_default_model", { model: defaultModel });
        console.log("Model updated:", defaultModel);
      } catch (err) {
        console.error("Failed to save model:", err);
      }
    }, 500);
  }

  function openNewTemplate() {
    editingTemplate = null;
    showEditor = true;
  }

  /**
   * @param {any} template
   */
  function openEditTemplate(template) {
    editingTemplate = template;
    showEditor = true;
  }

  /**
   * @param {any} template
   */
  async function handleTemplateSave(template) {
    showEditor = false;
    editingTemplate = null;
    await loadSettings(); // Reload templates
  }

  function handleTemplateCancel() {
    showEditor = false;
    editingTemplate = null;
  }

  /**
   * @param {string} templateId
   */
  async function handleDeleteTemplate(templateId) {
    if (!confirm("Are you sure you want to delete this template?")) {
      return;
    }

    try {
      await invoke("delete_template", { id: templateId });
      await loadSettings(); // Reload templates
    } catch (err) {
      console.error("Failed to delete template:", err);
      alert(`Failed to delete template: ${err}`);
    }
  }

  async function handleAutoLaunchToggle() {
    autoLaunchLoading = true;
    try {
      if (autoLaunchEnabled) {
        await invoke("disable_autostart");
        autoLaunchEnabled = false;
      } else {
        await invoke("enable_autostart");
        autoLaunchEnabled = true;
      }
    } catch (err) {
      console.error("Failed to toggle auto-launch:", err);
      alert(`Failed to change auto-launch setting: ${err}`);
    } finally {
      autoLaunchLoading = false;
    }
  }

  // Load settings on mount
  loadSettings();
</script>

<main class="container">
  <div class="header">
    <h1>Settings</h1>
    <a href="/" class="back-link">← Back to Home</a>
  </div>

  <p class="subtitle">Configure your API key, prompt templates, and hotkeys.</p>

  <div class="settings-sections">
    <!-- API Key Section -->
    <ApiKeyInput />

    <!-- Model Selection Section -->
    <section class="settings-section">
      <h2>Default Model</h2>
      <p class="description">Choose the Claude model to use for processing prompts.</p>
      <select bind:value={defaultModel} onchange={handleModelChange} class="model-select">
        <option value="claude-3-5-haiku-20241022">Claude 3.5 Haiku (Fast & Cost-Effective)</option>
        <option value="claude-3-5-sonnet-20241022">Claude 3.5 Sonnet (Balanced)</option>
        <option value="claude-3-opus-20240229">Claude 3 Opus (Most Capable)</option>
      </select>
    </section>

    <!-- Application Settings Section -->
    <section class="settings-section">
      <h2>Application Settings</h2>
      <p class="description">Configure application behavior and preferences.</p>

      <div class="setting-item">
        <div class="setting-info">
          <strong>Launch at Login</strong>
          <p class="setting-hint">Automatically start the app when you log in to your computer.</p>
        </div>
        <label class="toggle-switch">
          <input
            type="checkbox"
            checked={autoLaunchEnabled}
            onchange={handleAutoLaunchToggle}
            disabled={autoLaunchLoading}
          />
          <span class="toggle-slider"></span>
        </label>
      </div>
    </section>

    <!-- Prompt Templates Section -->
    <section class="settings-section">
      <h2>Prompt Templates</h2>
      <p class="description">Create and manage prompt templates with custom hotkeys.</p>

      {#if templates.length === 0}
        <div class="empty-state">
          <p>No templates yet. Create your first template to get started!</p>
          <button class="btn btn-primary" onclick={openNewTemplate}>+ Add Template</button>
        </div>
      {:else}
        <div class="templates-list">
          {#each templates as template (template.id)}
            <div class="template-item">
              <div class="template-header">
                <h3>{template.name}</h3>
                {#if template.hotkey}
                  <span class="hotkey-badge">{template.hotkey}</span>
                {/if}
              </div>
              <p class="template-description">{template.description || "No description"}</p>
              <div class="template-preview">
                <code>{template.prompt.substring(0, 100)}{template.prompt.length > 100 ? '...' : ''}</code>
              </div>
              <div class="template-actions">
                <button class="btn-small btn-secondary" onclick={() => openEditTemplate(template)}>Edit</button>
                <button class="btn-small btn-danger" onclick={() => handleDeleteTemplate(template.id)}>Delete</button>
              </div>
            </div>
          {/each}
        </div>
        <button class="btn btn-primary" onclick={openNewTemplate}>+ Add Template</button>
      {/if}
    </section>
  </div>
</main>

{#if showEditor}
  <TemplateEditor
    template={editingTemplate}
    onSave={handleTemplateSave}
    onCancel={handleTemplateCancel}
  />
{/if}

<style>
  .container {
    padding: 2rem;
    max-width: 900px;
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

  .settings-sections {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .settings-section {
    padding: 1.5rem;
    border: 1px solid #ddd;
    border-radius: 8px;
    background: #fff;
  }

  .settings-section h2 {
    margin-top: 0;
    margin-bottom: 0.5rem;
    font-size: 1.25rem;
  }

  .description {
    color: #666;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .model-select {
    width: 100%;
    padding: 0.6rem 1rem;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 0.95rem;
    background: #fff;
    cursor: pointer;
  }

  .model-select:focus {
    outline: none;
    border-color: #646cff;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    background: #f9f9f9;
    border-radius: 6px;
  }

  .empty-state p {
    color: #666;
    margin-bottom: 1rem;
  }

  .templates-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .template-item {
    padding: 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 6px;
    background: #fafafa;
  }

  .template-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .template-header h3 {
    margin: 0;
    font-size: 1.1rem;
  }

  .hotkey-badge {
    background: #646cff;
    color: white;
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .template-description {
    color: #666;
    font-size: 0.9rem;
    margin-bottom: 0.5rem;
  }

  .template-preview {
    background: #f5f5f5;
    padding: 0.5rem;
    border-radius: 4px;
    margin-bottom: 0.75rem;
    font-size: 0.85rem;
    overflow: hidden;
  }

  .template-preview code {
    color: #333;
    font-family: 'Courier New', monospace;
  }

  .template-actions {
    display: flex;
    gap: 0.5rem;
  }

  .btn, .btn-small {
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn {
    padding: 0.6rem 1.2rem;
  }

  .btn-small {
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
  }

  .btn-primary {
    background: #646cff;
    color: white;
  }

  .btn-primary:hover {
    background: #535ac8;
  }

  .btn-secondary {
    background: #f5f5f5;
    color: #333;
    border: 1px solid #ddd;
  }

  .btn-secondary:hover {
    background: #e5e5e5;
  }

  .btn-danger {
    background: #ff4444;
    color: white;
  }

  .btn-danger:hover {
    background: #cc0000;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 0;
    border-bottom: 1px solid #e0e0e0;
  }

  .setting-item:last-child {
    border-bottom: none;
  }

  .setting-info {
    flex: 1;
  }

  .setting-info strong {
    display: block;
    margin-bottom: 0.25rem;
    color: #333;
  }

  .setting-hint {
    margin: 0;
    font-size: 0.85rem;
    color: #666;
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    display: inline-block;
    width: 50px;
    height: 26px;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #ccc;
    transition: 0.3s;
    border-radius: 26px;
  }

  .toggle-slider:before {
    position: absolute;
    content: "";
    height: 20px;
    width: 20px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: 0.3s;
    border-radius: 50%;
  }

  .toggle-switch input:checked + .toggle-slider {
    background-color: #646cff;
  }

  .toggle-switch input:checked + .toggle-slider:before {
    transform: translateX(24px);
  }

  .toggle-switch input:disabled + .toggle-slider {
    opacity: 0.5;
    cursor: not-allowed;
  }

  @media (prefers-color-scheme: dark) {
    .subtitle {
      color: #aaa;
    }

    .settings-section {
      background: #1a1a1a;
      border-color: #444;
    }

    .description {
      color: #aaa;
    }

    .model-select {
      background: #2a2a2a;
      border-color: #444;
      color: #fff;
    }

    .empty-state {
      background: #2a2a2a;
    }

    .empty-state p {
      color: #aaa;
    }

    .template-item {
      background: #2a2a2a;
      border-color: #444;
    }

    .template-description {
      color: #aaa;
    }

    .template-preview {
      background: #2a2a2a;
    }

    .template-preview code {
      color: #ddd;
    }

    .btn-secondary {
      background: #2a2a2a;
      color: #fff;
      border-color: #444;
    }

    .btn-secondary:hover {
      background: #333;
    }

    .setting-item {
      border-bottom-color: #333;
    }

    .setting-info strong {
      color: #fff;
    }

    .setting-hint {
      color: #aaa;
    }

    .toggle-slider {
      background-color: #444;
    }
  }
</style>
