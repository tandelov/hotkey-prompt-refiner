import { y as attr, F as ensure_array_like } from "../../../chunks/index.js";
import { e as escape_html } from "../../../chunks/context.js";
import { invoke } from "@tauri-apps/api/core";
function ApiKeyInput($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let apiKey = "";
    let savedKey = "";
    let status = "idle";
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
    loadApiKey();
    $$renderer2.push(`<div class="api-key-section svelte-1t37msd"><h2 class="svelte-1t37msd">Anthropic API Key</h2> <p class="description svelte-1t37msd">Your API key is stored securely in the system keychain.
    Get your key from <a href="https://console.anthropic.com/settings/keys" target="_blank" class="svelte-1t37msd">Anthropic Console</a>.</p> <div class="input-group svelte-1t37msd"><div class="input-wrapper svelte-1t37msd"><input${attr("type", "password")}${attr("value", apiKey)} placeholder="sk-ant-..." class="api-key-input svelte-1t37msd"${attr("disabled", status === "testing", true)}/> <button class="toggle-visibility svelte-1t37msd" type="button"${attr("title", "Show key")}>${escape_html("👁️")}</button></div> <div class="button-group svelte-1t37msd"><button${attr("disabled", !apiKey.trim(), true)} class="btn btn-primary svelte-1t37msd">${escape_html("Save")}</button> <button${attr("disabled", !apiKey.trim() && !savedKey, true)} class="btn btn-secondary svelte-1t37msd">${escape_html("Test")}</button> <button${attr("disabled", !savedKey, true)} class="btn btn-danger svelte-1t37msd">Delete</button></div></div> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let templates = [];
    let defaultModel = "claude-3-5-haiku-20241022";
    let modelChangeTimeout = null;
    let autoLaunchEnabled = false;
    let autoLaunchLoading = false;
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
      modelChangeTimeout = setTimeout(
        async () => {
          try {
            await invoke("set_default_model", { model: defaultModel });
            console.log("Model updated:", defaultModel);
          } catch (err) {
            console.error("Failed to save model:", err);
          }
        },
        500
      );
    }
    loadSettings();
    $$renderer2.push(`<main class="container svelte-1i19ct2"><div class="header svelte-1i19ct2"><h1 class="svelte-1i19ct2">Settings</h1> <a href="/" class="back-link svelte-1i19ct2">← Back to Home</a></div> <p class="subtitle svelte-1i19ct2">Configure your API key, prompt templates, and hotkeys.</p> <div class="settings-sections svelte-1i19ct2">`);
    ApiKeyInput($$renderer2);
    $$renderer2.push(`<!----> <section class="settings-section svelte-1i19ct2"><h2 class="svelte-1i19ct2">Default Model</h2> <p class="description svelte-1i19ct2">Choose the Claude model to use for processing prompts.</p> `);
    $$renderer2.select(
      {
        value: defaultModel,
        onchange: handleModelChange,
        class: "model-select"
      },
      ($$renderer3) => {
        $$renderer3.option({ value: "claude-3-5-haiku-20241022" }, ($$renderer4) => {
          $$renderer4.push(`Claude 3.5 Haiku (Fast &amp; Cost-Effective)`);
        });
        $$renderer3.option({ value: "claude-3-5-sonnet-20241022" }, ($$renderer4) => {
          $$renderer4.push(`Claude 3.5 Sonnet (Balanced)`);
        });
        $$renderer3.option({ value: "claude-3-opus-20240229" }, ($$renderer4) => {
          $$renderer4.push(`Claude 3 Opus (Most Capable)`);
        });
      },
      "svelte-1i19ct2"
    );
    $$renderer2.push(`</section> <section class="settings-section svelte-1i19ct2"><h2 class="svelte-1i19ct2">Application Settings</h2> <p class="description svelte-1i19ct2">Configure application behavior and preferences.</p> <div class="setting-item svelte-1i19ct2"><div class="setting-info svelte-1i19ct2"><strong class="svelte-1i19ct2">Launch at Login</strong> <p class="setting-hint svelte-1i19ct2">Automatically start the app when you log in to your computer.</p></div> <label class="toggle-switch svelte-1i19ct2"><input type="checkbox"${attr("checked", autoLaunchEnabled, true)}${attr("disabled", autoLaunchLoading, true)} class="svelte-1i19ct2"/> <span class="toggle-slider svelte-1i19ct2"></span></label></div></section> <section class="settings-section svelte-1i19ct2"><h2 class="svelte-1i19ct2">Prompt Templates</h2> <p class="description svelte-1i19ct2">Create and manage prompt templates with custom hotkeys.</p> `);
    if (templates.length === 0) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="empty-state svelte-1i19ct2"><p class="svelte-1i19ct2">No templates yet. Create your first template to get started!</p> <button class="btn btn-primary svelte-1i19ct2">+ Add Template</button></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<div class="templates-list svelte-1i19ct2"><!--[-->`);
      const each_array = ensure_array_like(templates);
      for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
        let template = each_array[$$index];
        $$renderer2.push(`<div class="template-item svelte-1i19ct2"><div class="template-header svelte-1i19ct2"><h3 class="svelte-1i19ct2">${escape_html(template.name)}</h3> `);
        if (template.hotkey) {
          $$renderer2.push("<!--[-->");
          $$renderer2.push(`<span class="hotkey-badge svelte-1i19ct2">${escape_html(template.hotkey)}</span>`);
        } else {
          $$renderer2.push("<!--[!-->");
        }
        $$renderer2.push(`<!--]--></div> <p class="template-description svelte-1i19ct2">${escape_html(template.description || "No description")}</p> <div class="template-preview svelte-1i19ct2"><code class="svelte-1i19ct2">${escape_html(template.prompt.substring(0, 100))}${escape_html(template.prompt.length > 100 ? "..." : "")}</code></div> <div class="template-actions svelte-1i19ct2"><button class="btn-small btn-secondary svelte-1i19ct2">Edit</button> <button class="btn-small btn-danger svelte-1i19ct2">Delete</button></div></div>`);
      }
      $$renderer2.push(`<!--]--></div> <button class="btn btn-primary svelte-1i19ct2">+ Add Template</button>`);
    }
    $$renderer2.push(`<!--]--></section></div></main> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]-->`);
  });
}
export {
  _page as default
};
