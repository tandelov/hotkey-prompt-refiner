<script>
  import { invoke } from "@tauri-apps/api/core";

  // Props
  let { template = null, onSave, onCancel } = $props();

  // Form state
  let id = $state(template?.id || "");
  let name = $state(template?.name || "");
  let description = $state(template?.description || "");
  let prompt = $state(template?.prompt || "");
  let hotkey = $state(template?.hotkey || "");

  let status = $state("idle"); // idle, saving, error
  let message = $state("");
  let isEdit = $state(!!template);

  async function handleSave() {
    // Validation
    if (!name.trim()) {
      status = "error";
      message = "Template name is required";
      return;
    }

    if (!prompt.trim()) {
      status = "error";
      message = "Prompt text is required";
      return;
    }

    // Check if prompt contains {clipboard_text} placeholder
    if (!prompt.includes("{clipboard_text}")) {
      if (!confirm("Your prompt doesn't contain {clipboard_text} placeholder. Continue anyway?")) {
        return;
      }
    }

    status = "saving";
    message = "";

    try {
      const templateData = {
        id: id || crypto.randomUUID(),
        name: name.trim(),
        description: description.trim(),
        prompt: prompt.trim(),
        hotkey: hotkey.trim() || null,
        created_at: template?.created_at || new Date().toISOString(),
      };

      await invoke("save_template", { template: templateData });

      status = "idle";
      if (onSave) {
        onSave(templateData);
      }
    } catch (err) {
      status = "error";
      message = `Failed to save template: ${err}`;
    }
  }

  function handleCancel() {
    if (onCancel) {
      onCancel();
    }
  }
</script>

<div class="modal-backdrop" onclick={handleCancel} onkeydown={(e) => e.key === 'Escape' && handleCancel()} role="button" tabindex="-1" aria-label="Close modal">
  <div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="0">
    <div class="modal-header">
      <h2>{isEdit ? "Edit Template" : "Create New Template"}</h2>
      <button class="close-button" onclick={handleCancel}>&times;</button>
    </div>

    <div class="modal-body">
      <div class="form-group">
        <label for="name">Template Name *</label>
        <input
          id="name"
          type="text"
          bind:value={name}
          placeholder="e.g., Fix Grammar"
          class="input"
          disabled={status === "saving"}
        />
      </div>

      <div class="form-group">
        <label for="description">Description</label>
        <input
          id="description"
          type="text"
          bind:value={description}
          placeholder="Brief description of what this template does"
          class="input"
          disabled={status === "saving"}
        />
      </div>

      <div class="form-group">
        <label for="prompt">Prompt Template *</label>
        <textarea
          id="prompt"
          bind:value={prompt}
          placeholder="Enter your prompt here. Use &#123;clipboard_text&#125; to insert clipboard content."
          class="textarea"
          rows="8"
          disabled={status === "saving"}
        ></textarea>
        <span class="hint">
          Use <code>&#123;clipboard_text&#125;</code> to insert clipboard content in your prompt.
        </span>
      </div>

      <div class="form-group">
        <label for="hotkey">Hotkey (Optional)</label>
        <input
          id="hotkey"
          type="text"
          bind:value={hotkey}
          placeholder="e.g., Cmd+Shift+G or Ctrl+Alt+F"
          class="input"
          disabled={status === "saving"}
        />
        <span class="hint">
          Leave empty to assign later. Format: Cmd/Ctrl+Shift+Key
        </span>
      </div>

      {#if message}
        <div class="message" class:error={status === "error"}>
          {message}
        </div>
      {/if}
    </div>

    <div class="modal-footer">
      <button
        class="btn btn-secondary"
        onclick={handleCancel}
        disabled={status === "saving"}
      >
        Cancel
      </button>
      <button
        class="btn btn-primary"
        onclick={handleSave}
        disabled={status === "saving"}
      >
        {status === "saving" ? "Saving..." : (isEdit ? "Update" : "Create")}
      </button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal {
    background: #fff;
    border-radius: 12px;
    width: 100%;
    max-width: 600px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #e0e0e0;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.5rem;
  }

  .close-button {
    background: none;
    border: none;
    font-size: 2rem;
    line-height: 1;
    cursor: pointer;
    color: #666;
    padding: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: all 0.2s;
  }

  .close-button:hover {
    background: #f0f0f0;
    color: #333;
  }

  .modal-body {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }

  .form-group {
    margin-bottom: 1.25rem;
  }

  .form-group:last-child {
    margin-bottom: 0;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #333;
  }

  .input, .textarea {
    width: 100%;
    padding: 0.6rem 1rem;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.95rem;
    transition: border-color 0.2s;
  }

  .input:focus, .textarea:focus {
    outline: none;
    border-color: #646cff;
  }

  .input:disabled, .textarea:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }

  .textarea {
    resize: vertical;
    font-family: 'Courier New', monospace;
  }

  .hint {
    display: block;
    margin-top: 0.4rem;
    font-size: 0.85rem;
    color: #666;
  }

  .hint code {
    background: #f5f5f5;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: 'Courier New', monospace;
  }

  .message {
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.9rem;
    margin-top: 1rem;
  }

  .message.error {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1.5rem;
    border-top: 1px solid #e0e0e0;
  }

  .btn {
    padding: 0.6rem 1.5rem;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.95rem;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #646cff;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #535ac8;
  }

  .btn-secondary {
    background: #f5f5f5;
    color: #333;
    border: 1px solid #ddd;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #e5e5e5;
  }

  @media (prefers-color-scheme: dark) {
    .modal {
      background: #1a1a1a;
    }

    .modal-header {
      border-bottom-color: #333;
    }

    .close-button {
      color: #aaa;
    }

    .close-button:hover {
      background: #2a2a2a;
      color: #fff;
    }

    label {
      color: #fff;
    }

    .input, .textarea {
      background: #2a2a2a;
      border-color: #444;
      color: #fff;
    }

    .input:disabled, .textarea:disabled {
      background-color: #1a1a1a;
    }

    .hint {
      color: #aaa;
    }

    .hint code {
      background: #2a2a2a;
    }

    .modal-footer {
      border-top-color: #333;
    }

    .btn-secondary {
      background: #2a2a2a;
      color: #fff;
      border-color: #444;
    }

    .btn-secondary:hover:not(:disabled) {
      background: #333;
    }
  }
</style>
