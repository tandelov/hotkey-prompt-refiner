import { y as attr, F as ensure_array_like } from "../../../chunks/index.js";
import { invoke } from "@tauri-apps/api/core";
import { e as escape_html } from "../../../chunks/context.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let history = [];
    let searchQuery = "";
    let expandedId = null;
    let loading = true;
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
    function formatTime(timestamp) {
      const date = new Date(timestamp);
      const now = /* @__PURE__ */ new Date();
      const diff = now.getTime() - date.getTime();
      const seconds = Math.floor(diff / 1e3);
      const minutes = Math.floor(seconds / 60);
      const hours = Math.floor(minutes / 60);
      const days = Math.floor(hours / 24);
      if (days > 0) return `${days}d ago`;
      if (hours > 0) return `${hours}h ago`;
      if (minutes > 0) return `${minutes}m ago`;
      return `${seconds}s ago`;
    }
    loadHistory();
    $$renderer2.push(`<main class="container svelte-1xl2tfr"><div class="header svelte-1xl2tfr"><h1 class="svelte-1xl2tfr">History</h1> <a href="/" class="back-link svelte-1xl2tfr">← Back to Home</a></div> <p class="subtitle svelte-1xl2tfr">View your recent prompt processing history (last 100 entries).</p> <div class="controls svelte-1xl2tfr"><input type="text"${attr("value", searchQuery)} placeholder="Search by template name or content..." class="search-input svelte-1xl2tfr"/> <button${attr("disabled", history.length === 0, true)} class="btn btn-danger svelte-1xl2tfr">Clear All</button></div> `);
    if (loading) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="loading svelte-1xl2tfr">Loading history...</div>`);
    } else {
      $$renderer2.push("<!--[!-->");
      if (history.length === 0) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<div class="empty-state svelte-1xl2tfr"><p>No history entries yet.</p> <p class="hint svelte-1xl2tfr">History will appear here after you process text using hotkeys.</p></div>`);
      } else {
        $$renderer2.push("<!--[!-->");
        $$renderer2.push(`<div class="history-list svelte-1xl2tfr"><!--[-->`);
        const each_array = ensure_array_like(history);
        for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
          let entry = each_array[$$index];
          $$renderer2.push(`<div class="history-item svelte-1xl2tfr"><div class="history-header svelte-1xl2tfr"><div class="history-info svelte-1xl2tfr"><span class="template-name svelte-1xl2tfr">${escape_html(entry.template_name)}</span> <span class="timestamp svelte-1xl2tfr">${escape_html(formatTime(entry.timestamp))}</span></div> <button class="expand-btn svelte-1xl2tfr"${attr("aria-label", expandedId === entry.timestamp ? "Collapse" : "Expand")}>${escape_html(expandedId === entry.timestamp ? "−" : "+")}</button></div> <div class="preview-section svelte-1xl2tfr"><div class="preview-item svelte-1xl2tfr"><span class="preview-label svelte-1xl2tfr">Input:</span> <span class="preview-text svelte-1xl2tfr">${escape_html(entry.source_preview)}</span></div> <div class="preview-item svelte-1xl2tfr"><span class="preview-label svelte-1xl2tfr">Output:</span> <span class="preview-text svelte-1xl2tfr">${escape_html(entry.result_preview)}</span></div></div> `);
          if (expandedId === entry.timestamp) {
            $$renderer2.push("<!--[-->");
            $$renderer2.push(`<div class="expanded-content svelte-1xl2tfr"><div class="full-content svelte-1xl2tfr"><div class="content-header svelte-1xl2tfr"><strong class="svelte-1xl2tfr">Full Input</strong> <button class="copy-btn svelte-1xl2tfr">Copy</button></div> <pre class="content-text svelte-1xl2tfr">${escape_html(entry.full_source)}</pre></div> <div class="full-content svelte-1xl2tfr"><div class="content-header svelte-1xl2tfr"><strong class="svelte-1xl2tfr">Full Output</strong> <button class="copy-btn svelte-1xl2tfr">Copy</button></div> <pre class="content-text svelte-1xl2tfr">${escape_html(entry.full_result)}</pre></div></div>`);
          } else {
            $$renderer2.push("<!--[!-->");
          }
          $$renderer2.push(`<!--]--></div>`);
        }
        $$renderer2.push(`<!--]--></div>`);
      }
      $$renderer2.push(`<!--]-->`);
    }
    $$renderer2.push(`<!--]--></main>`);
  });
}
export {
  _page as default
};
