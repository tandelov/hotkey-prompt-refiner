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
    if (modelChangeTimeout) {
      clearTimeout(modelChangeTimeout);
    }

    modelChangeTimeout = setTimeout(async () => {
      try {
        await invoke("set_default_model", { model: defaultModel });
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
    await loadSettings();
  }

  function handleTemplateCancel() {
    showEditor = false;
    editingTemplate = null;
  }

  /**
   * @param {string} templateId
   */
  async function handleDeleteTemplate(templateId) {
    if (!confirm("Delete this template?")) {
      return;
    }

    try {
      await invoke("delete_template", { id: templateId });
      await loadSettings();
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
      alert(`Failed to change auto-launch: ${err}`);
    } finally {
      autoLaunchLoading = false;
    }
  }

  loadSettings();
</script>

<div class="settings-page">
  <div class="page-header">
    <h1>Settings</h1>
  </div>

  <div class="content-area">
    <!-- API Key Section -->
    <div class="section">
      <div class="section-header">
        <h2>API Configuration</h2>
      </div>
      <ApiKeyInput />
    </div>

    <!-- Model & Options -->
    <div class="section">
      <div class="section-header">
        <h2>Model & Preferences</h2>
      </div>
      <div class="form-row">
        <label for="model-select">Default Model</label>
        <select id="model-select" bind:value={defaultModel} onchange={handleModelChange}>
          <option value="claude-3-5-haiku-20241022">Claude 3.5 Haiku</option>
          <option value="claude-3-5-sonnet-20241022">Claude 3.5 Sonnet</option>
          <option value="claude-3-opus-20240229">Claude 3 Opus</option>
        </select>
      </div>

      <div class="form-row toggle-row">
        <div class="toggle-label">
          <span>Launch at Login</span>
          <span class="hint">Start app when you log in</span>
        </div>
        <label class="toggle-switch">
          <input
            type="checkbox"
            checked={autoLaunchEnabled}
            onchange={handleAutoLaunchToggle}
            disabled={autoLaunchLoading}
          />
          <span class="slider"></span>
        </label>
      </div>
    </div>

    <!-- Templates Section -->
    <div class="section section-grow">
      <div class="section-header">
        <h2>Templates</h2>
        <button class="btn-add" onclick={openNewTemplate}>+</button>
      </div>

      {#if templates.length === 0}
        <div class="empty-templates">
          <p>No templates yet</p>
          <button onclick={openNewTemplate}>Create Template</button>
        </div>
      {:else}
        <div class="templates-scroll">
          {#each templates as template (template.id)}
            <div class="template-row">
              <div class="template-info">
                <div class="template-name">{template.name}</div>
                {#if template.hotkey}
                  <div class="template-hotkey">{template.hotkey}</div>
                {/if}
              </div>
              <div class="template-actions">
                <button class="btn-icon" onclick={() => openEditTemplate(template)} title="Edit">✎</button>
                <button class="btn-icon btn-danger" onclick={() => handleDeleteTemplate(template.id)} title="Delete">×</button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

{#if showEditor}
  <TemplateEditor
    template={editingTemplate}
    onSave={handleTemplateSave}
    onCancel={handleTemplateCancel}
  />
{/if}

<style>
  .settings-page {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .page-header {
    padding: 20px 24px 16px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    flex-shrink: 0;
  }

  .page-header h1 {
    margin: 0;
    font-size: 22px;
    font-weight: 600;
    color: #1d1d1f;
  }

  .content-area {
    flex: 1;
    overflow-y: auto;
    padding: 16px 24px 24px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .section {
    flex-shrink: 0;
  }

  .section-grow {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .section-header h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
    color: #1d1d1f;
  }

  .form-row {
    display: flex;
    align-items: center;
    padding: 10px 0;
    border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  }

  .form-row:last-child {
    border-bottom: none;
  }

  .form-row label {
    flex: 0 0 140px;
    font-size: 13px;
    color: #1d1d1f;
  }

  .form-row select {
    flex: 1;
    padding: 5px 8px;
    border: 1px solid rgba(0, 0, 0, 0.2);
    border-radius: 6px;
    background: #ffffff;
    font-size: 13px;
    font-family: inherit;
    color: #1d1d1f;
  }

  .form-row select:focus {
    outline: none;
    border-color: #007aff;
  }

  .toggle-row {
    justify-content: space-between;
  }

  .toggle-label {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toggle-label span:first-child {
    font-size: 13px;
    color: #1d1d1f;
  }

  .hint {
    font-size: 11px;
    color: #86868b;
  }

  .toggle-switch {
    position: relative;
    display: inline-block;
    width: 42px;
    height: 26px;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #e0e0e0;
    transition: 0.2s;
    border-radius: 26px;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 22px;
    width: 22px;
    left: 2px;
    bottom: 2px;
    background-color: white;
    transition: 0.2s;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .toggle-switch input:checked + .slider {
    background-color: #34c759;
  }

  .toggle-switch input:checked + .slider:before {
    transform: translateX(16px);
  }

  .toggle-switch input:disabled + .slider {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-add {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    border: none;
    background: #007aff;
    color: white;
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
  }

  .btn-add:hover {
    background: #0051d5;
  }

  .empty-templates {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 40px 20px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 8px;
  }

  .empty-templates p {
    margin: 0;
    font-size: 13px;
    color: #86868b;
  }

  .empty-templates button {
    padding: 6px 16px;
    border: none;
    border-radius: 6px;
    background: #007aff;
    color: white;
    font-size: 13px;
    font-family: inherit;
    cursor: pointer;
  }

  .empty-templates button:hover {
    background: #0051d5;
  }

  .templates-scroll {
    flex: 1;
    overflow-y: auto;
    margin: -4px;
    padding: 4px;
  }

  .template-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    border-radius: 6px;
    background: rgba(0, 0, 0, 0.03);
    margin-bottom: 6px;
  }

  .template-row:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .template-info {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .template-name {
    font-size: 13px;
    color: #1d1d1f;
    font-weight: 500;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .template-hotkey {
    font-size: 11px;
    padding: 3px 8px;
    background: rgba(0, 122, 255, 0.15);
    color: #007aff;
    border-radius: 4px;
    font-weight: 500;
    flex-shrink: 0;
  }

  .template-actions {
    display: flex;
    gap: 4px;
  }

  .btn-icon {
    width: 28px;
    height: 28px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: #1d1d1f;
    font-size: 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
  }

  .btn-icon:hover {
    background: rgba(0, 0, 0, 0.08);
  }

  .btn-icon.btn-danger {
    color: #ff3b30;
  }

  .btn-icon.btn-danger:hover {
    background: rgba(255, 59, 48, 0.1);
  }

  @media (prefers-color-scheme: dark) {
    .page-header {
      border-bottom-color: rgba(255, 255, 255, 0.1);
    }

    .page-header h1,
    .section-header h2,
    .form-row label,
    .toggle-label span:first-child,
    .template-name,
    .btn-icon {
      color: #f5f5f7;
    }

    .form-row {
      border-bottom-color: rgba(255, 255, 255, 0.08);
    }

    .form-row select {
      background: rgba(255, 255, 255, 0.1);
      border-color: rgba(255, 255, 255, 0.2);
      color: #f5f5f7;
    }

    .slider {
      background-color: #3a3a3c;
    }

    .empty-templates {
      background: rgba(255, 255, 255, 0.05);
    }

    .empty-templates p {
      color: #98989d;
    }

    .template-row {
      background: rgba(255, 255, 255, 0.05);
    }

    .template-row:hover {
      background: rgba(255, 255, 255, 0.08);
    }

    .template-hotkey {
      background: rgba(10, 132, 255, 0.25);
      color: #0a84ff;
    }

    .btn-icon:hover {
      background: rgba(255, 255, 255, 0.1);
    }

    .btn-icon.btn-danger:hover {
      background: rgba(255, 69, 58, 0.2);
    }
  }
</style>
