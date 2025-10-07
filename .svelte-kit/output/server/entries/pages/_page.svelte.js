import { x as attr_class, y as attr, z as stringify } from "../../chunks/index.js";
import { invoke } from "@tauri-apps/api/core";
import { e as escape_html } from "../../chunks/context.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let name = "";
    let greetMsg = "";
    let templates = [];
    let activeHotkeys = 0;
    async function loadStatus() {
      try {
        templates = await invoke("get_templates");
        activeHotkeys = templates.filter((t) => t.hotkey).length;
      } catch (err) {
        console.error("Failed to load status:", err);
      }
    }
    loadStatus();
    $$renderer2.push(`<main class="container svelte-1uha8ag"><h1 class="svelte-1uha8ag">Hotkey Prompt Refiner</h1> <nav class="nav svelte-1uha8ag"><a href="/settings" class="nav-link svelte-1uha8ag">Settings</a> <a href="/history" class="nav-link svelte-1uha8ag">History</a></nav> <div class="info svelte-1uha8ag"><p>AI-powered text processing via global hotkeys</p> <p class="hint svelte-1uha8ag">💡 Tip: Use the system tray icon to quickly access Settings and History</p></div> <div class="status-section svelte-1uha8ag"><h2 class="svelte-1uha8ag">Status</h2> <div class="status-card svelte-1uha8ag"><div class="status-item svelte-1uha8ag"><span class="status-label svelte-1uha8ag">Templates:</span> <span class="status-value svelte-1uha8ag">${escape_html(templates.length)}</span></div> <div class="status-item svelte-1uha8ag"><span class="status-label svelte-1uha8ag">Active Hotkeys:</span> <span${attr_class(`status-value ${stringify(activeHotkeys > 0 ? "active" : "inactive")}`, "svelte-1uha8ag")}>${escape_html(activeHotkeys)}</span></div> `);
    if (activeHotkeys === 0) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="status-hint svelte-1uha8ag">No hotkeys configured. <a href="/settings" class="svelte-1uha8ag">Go to Settings</a> to create templates.</div>`);
    } else {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<div class="status-hint success svelte-1uha8ag">✓ Hotkeys are active! Copy text and press your configured hotkey.</div>`);
    }
    $$renderer2.push(`<!--]--></div></div> <div class="demo-section svelte-1uha8ag"><h2 class="svelte-1uha8ag">Demo</h2> <form class="row svelte-1uha8ag"><input id="greet-input" placeholder="Enter a name..."${attr("value", name)} class="svelte-1uha8ag"/> <button type="submit" class="svelte-1uha8ag">Greet</button></form> <p>${escape_html(greetMsg)}</p></div></main>`);
  });
}
export {
  _page as default
};
