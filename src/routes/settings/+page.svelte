<script>
  import ApiKeyInput from '$lib/components/ApiKeyInput.svelte';
  import TemplateEditor from '$lib/components/TemplateEditor.svelte';
  import { invoke } from "@tauri-apps/api/core";

  /** @type {any[]} */
  let templates = $state([]);
  let defaultModel = $state("claude-sonnet-4-5-20250929");
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
          <option value="claude-sonnet-4-5-20250929">Claude Sonnet 4.5 (best performance, good speed)</option>
          <option value="claude-3-5-haiku-latest">Claude Haiku 3.5 (good performance, great speed)</option>
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
      </div>

      <div class="templates-container">
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

        <div class="list-footer-bar">
          <button class="btn-add" onclick={openNewTemplate} title="Add Template">
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19"></line>
              <line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
          </button>
        </div>
      </div>
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
    margin-top: 40px;
    margin-bottom: 16px;
  }

  .section-header:first-of-type {
    margin-top: 20px;
  }

  .section-header h2 {
    margin: 0;
    font-size: 20px;
    font-weight: 600;
    color: #1d1d1f;
  }

  .form-row {
    display: grid;
    grid-template-columns: 150px 1fr;
    align-items: center;
    gap: 16px;
    padding: 10px 0;
    border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  }

  .form-row:last-child {
    border-bottom: none;
  }

  .form-row label {
    justify-self: end;
    font-size: 14px;
    color: #1d1d1f;
  }

  .form-row select {
    /* Reset default button styles for native appearance */
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none;

    /* Layout */
    padding: 6px 30px 6px 12px;

    /* Visual styling to match NSPopUpButton */
    background-color: rgba(0, 0, 0, 0.05);
    background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 12 12"><path fill="%239A9A9A" d="M6 8L2 4h8z"/></svg>');
    background-repeat: no-repeat;
    background-position: right 8px center;
    background-size: 12px;
    border: 1px solid rgba(0, 0, 0, 0.15);
    border-radius: 6px;

    /* Typography */
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 14px;
    color: #1d1d1f;

    /* Interaction */
    cursor: default;
    text-align: left;
  }

  .form-row select:focus {
    outline: none;
    border-color: #007AFF;
    box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.4);
  }

  .toggle-row {
    grid-template-columns: 1fr auto;
    gap: 12px;
  }

  .toggle-label {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .toggle-label span:first-child {
    font-size: 14px;
    color: #1d1d1f;
  }

  .hint {
    font-size: 11px;
    color: #86868b;
  }

  .toggle-switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 26px;
    flex-shrink: 0;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
    position: absolute;
  }

  /* macOS-style toggle switch track */
  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.15);
    transition: background-color 0.2s ease-in-out;
    border-radius: 13px;
  }

  /* macOS-style toggle switch knob */
  .slider:before {
    position: absolute;
    content: "";
    height: 22px;
    width: 22px;
    left: 2px;
    top: 2px;
    background-color: white;
    transition: transform 0.2s ease-in-out;
    border-radius: 50%;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.15);
  }

  /* "On" state styling */
  .toggle-switch input:checked + .slider {
    background-color: #32D74B;
  }

  .toggle-switch input:checked + .slider:before {
    transform: translateX(18px);
  }

  /* Disabled state */
  .toggle-switch input:disabled + .slider {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .templates-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .list-footer-bar {
    padding-top: 8px;
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    flex-shrink: 0;
  }

  .btn-add {
    width: 24px;
    height: 24px;
    border-radius: 6px;
    border: 1px solid rgba(0, 0, 0, 0.15);
    background: transparent;
    color: #6e6e6e;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
  }

  .btn-add:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .btn-add svg {
    width: 14px;
    height: 14px;
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
    padding: 8px 16px;
    border: none;
    border-radius: 8px;
    background: #007AFF;
    color: white;
    font-size: 14px;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
  }

  .empty-templates button:hover {
    background: #0051D5;
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

    .page-header h1 {
      color: #f5f5f7;
    }

    .section-header h2 {
      color: #E0E0E0;
    }

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
      background-color: rgba(255, 255, 255, 0.1);
      background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 12 12"><path fill="%23CFCFCF" d="M6 8L2 4h8z"/></svg>');
      border-color: rgba(255, 255, 255, 0.15);
      color: #E0E0E0;
    }

    .slider {
      background-color: rgba(255, 255, 255, 0.2);
    }

    .toggle-switch input:checked + .slider {
      background-color: #32D74B;
    }

    .list-footer-bar {
      border-top-color: rgba(255, 255, 255, 0.1);
    }

    .btn-add {
      background: transparent;
      border-color: rgba(255, 255, 255, 0.15);
      color: #CFCFCF;
    }

    .btn-add:hover {
      background: rgba(255, 255, 255, 0.08);
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
